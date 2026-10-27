export interface RoundTS {
  roundid: string;
  /** The division the round belongs to. */
  did: string;
  name: string;
  scheduled_start_time: string | null;
  created_at: string;
  updated_at: string;
  /** One question type code per question (20), in question order; empty when none are saved. */
  question_types: string;
  /** Whether saving question types checks for the standard count of each type. */
  using_standard_question_types: boolean;
}

export interface NewRoundPayload {
  did: string;
  name: string;
  scheduled_start_time: string | null; // ISO 8601 datetime string; null when not scheduled
  question_types?: string;
  using_standard_question_types?: boolean;
}

/** One question set read out of an imported file. */
export interface ParsedQuestionSetTS {
  name: string;
  /** The set's 20 question type codes, or empty when `errors` is not. */
  question_types: string;
  errors: string[];
}

export interface ParsedQuestionSetFileTS {
  sets: ParsedQuestionSetTS[];
  /** Problems with the file as a whole (e.g. an unsupported format). */
  errors: string[];
}

export interface RoundQuestionTypesTS {
  roundid: string;
  name: string;
  scheduled_start_time: string | null;
  question_types: string;
  using_standard_question_types: boolean;
}

export interface DivisionQuestionTypesTS {
  /** True only when every round of the division has a question type for every question. */
  complete: boolean;
  rounds: RoundQuestionTypesTS[];
}

export interface RoundQuestionTypesUpdate {
  roundid: string;
  question_types: string;
  using_standard_question_types: boolean;
}

/** Reads the error message out of a failed response, including per-round validation errors. */
async function errorMessage(response: Response, fallback: string): Promise<string> {
  try {
    const body = await response.json();
    const lines: string[] = [];
    if (body?.error) lines.push(body.error);
    if (Array.isArray(body?.errors)) lines.push(...body.errors);
    if (Array.isArray(body?.rounds)) {
      for (const r of body.rounds) lines.push(...(r.errors ?? []));
    }
    if (lines.length > 0) return lines.join(' ');
  } catch { /* non-JSON body */ }
  return `${fallback} (${response.status})`;
}

export interface PagedRounds {
  count: number;
  items: RoundTS[];
}

/**
 * One fully-formed row of the rounds data table: the round plus its division id/name, so the whole
 * table is populated from a single request per page.
 */
export interface RoundRowTS {
  roundid: string;
  did: string;
  division_name: string;
  name: string;
  scheduled_start_time: string | null;
  created_at: string;
  updated_at: string;
  last_modified_user_name: string;
  last_modified_user_id: string;
}

export interface PagedRoundRows {
  count: number;
  items: RoundRowTS[];
}

export const RoundAPI = {
  get: async (page: number, size: number): Promise<PagedRounds> =>
    (await fetch(`/api/rounds?page=${page}&page_size=${size}`)).json(),
  getByTournament: async (tid: string, page: number, size: number): Promise<RoundTS[]> =>
    (await fetch(`/api/tournaments/${tid}/rounds?page=${page}&page_size=${size}`)).json(),
  getByDivision: async (did: string, page: number, size: number): Promise<RoundTS[]> =>
    (await fetch(`/api/divisions/${did}/rounds?page=${page}&page_size=${size}`)).json(),
  /** One page of the tournament's enriched round rows (division name), plus total count. */
  getRowsByTournament: async (tid: string, page: number, size: number): Promise<PagedRoundRows> =>
    (await fetch(`/api/tournaments/${tid}/round-rows?page=${page}&page_size=${size}`)).json(),
  /** One page of the division's enriched round rows (division name), plus total count. */
  getRowsByDivision: async (did: string, page: number, size: number): Promise<PagedRoundRows> =>
    (await fetch(`/api/divisions/${did}/round-rows?page=${page}&page_size=${size}`)).json(),
  create: async (round: NewRoundPayload, accessToken?: string): Promise<RoundTS> => {
    const response = await fetch('/api/rounds', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(round),
    });
    if (!response.ok) {
      throw new Error(await errorMessage(response, 'Failed to create round'));
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  getById: async (id: string): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`);
    if (!response.ok) throw new Error(`Round not found (${response.status})`);
    return response.json();
  },
  /** Update a round's name, start time and/or question types. */
  update: async (id: string, payload: { name?: string; scheduled_start_time?: string | null; question_types?: string; using_standard_question_types?: boolean }, accessToken?: string): Promise<RoundTS> => {
    const response = await fetch(`/api/rounds/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(payload),
    });
    if (!response.ok) {
      throw new Error(await errorMessage(response, 'Failed to update round'));
    }
    const envelope = await response.json();
    return envelope.data ?? envelope;
  },
  /** Every round of the division with its question types, in schedule order. */
  getQuestionTypesByDivision: async (did: string): Promise<DivisionQuestionTypesTS> => {
    const response = await fetch(`/api/divisions/${did}/round-question-types`);
    if (!response.ok) throw new Error(`Failed to load question types (${response.status})`);
    return response.json();
  },
  /** Reads the question types out of a question set file (.rtf or .csv). Saves nothing. */
  parseQuestionSetFile: async (did: string, filename: string, content: string, accessToken?: string): Promise<ParsedQuestionSetFileTS> => {
    const response = await fetch(`/api/divisions/${did}/round-question-types/parse`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify({ filename, content }),
    });
    if (!response.ok) throw new Error(await errorMessage(response, 'Failed to read the file'));
    return response.json();
  },
  /** Saves several rounds' question types at once; all are saved or, on any error, none. */
  updateQuestionTypesOfDivision: async (did: string, updates: RoundQuestionTypesUpdate[], accessToken?: string): Promise<RoundTS[]> => {
    const response = await fetch(`/api/divisions/${did}/round-question-types`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}),
      },
      body: JSON.stringify(updates),
    });
    if (!response.ok) throw new Error(await errorMessage(response, 'Failed to save question types'));
    return response.json();
  },
  delete: async (id: string, accessToken?: string): Promise<void> => {
    const response = await fetch(`/api/rounds/${id}`, {
      method: 'DELETE',
      headers: { ...(accessToken ? { 'Authorization': `Bearer ${accessToken}` } : {}) },
    });
    if (!response.ok) {
      // Surface the server's message (e.g. the "a game is associated" guard) when present.
      let message = `Failed to delete round (${response.status})`;
      try {
        const body = await response.json();
        if (body?.error) message = body.error;
      } catch { /* non-JSON body */ }
      throw new Error(message);
    }
  },
}
