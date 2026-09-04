import type { UserTS } from './UserAPI';

/** A referenced entity (division or team) rendered as a link in the quizzers data table. */
export interface EntityRef {
  id: string;
  name: string;
}

/**
 * One fully-formed row of the quizzers data table: the quizzer's user fields plus the
 * divisions and teams they belong to within the requested scope. The whole table is
 * populated from a single request to one of the endpoints below.
 */
export interface QuizzerRowTS extends UserTS {
  divisions: EntityRef[];
  teams: EntityRef[];
}

export interface PagedQuizzerRows {
  count: number;
  items: QuizzerRowTS[];
}

export const QuizzerAPI = {
  /** One page of the tournament's quizzers (enriched), plus the total count. */
  getByTournament: async (tid: string, page: number, size: number): Promise<PagedQuizzerRows> =>
    (await fetch(`/api/tournaments/${tid}/quizzer-rows?page=${page}&page_size=${size}`)).json(),
  /** One page of the division's quizzers (enriched), plus the total count. */
  getByDivision: async (did: string, page: number, size: number): Promise<PagedQuizzerRows> =>
    (await fetch(`/api/divisions/${did}/quizzer-rows?page=${page}&page_size=${size}`)).json(),
};
