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

export const QuizzerAPI = {
  /** All quizzers rostered on any team in the tournament (enriched, one call). */
  getByTournament: async (tid: string): Promise<QuizzerRowTS[]> =>
    (await fetch(`/api/tournaments/${tid}/quizzer-rows`)).json(),
  /** All quizzers rostered on any team in the division (enriched, one call). */
  getByDivision: async (did: string): Promise<QuizzerRowTS[]> =>
    (await fetch(`/api/divisions/${did}/quizzer-rows`)).json(),
};
