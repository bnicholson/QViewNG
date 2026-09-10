export interface PoolBracketTS {
  pool_bracket_id: string;
  division_session_id: string;
  type: string;
  created_date: string;
  creator_userid: string;
  last_modified_date: string;
  last_modified_userid: string;
  name: string;
}

export const PoolBracketAPI = {
  /** All pool brackets in a division, across all of its division sessions. */
  getByDivision: async (did: string): Promise<PoolBracketTS[]> =>
    (await fetch(`/api/divisions/${did}/pool-brackets`)).json(),
}
