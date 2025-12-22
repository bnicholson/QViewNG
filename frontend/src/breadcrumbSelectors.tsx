import type { RootState } from "./store";

// Other code such as selectors can use the imported `RootState` type
export const selectDisplayDate = (state: RootState) => state.breadCrumb.displayDate;
export const selectIsOn = (state: RootState) => state.breadCrumb.isOn;
export const selectTournament = (state: RootState) => state.breadCrumb.tournament;
export const selectTid = (state: RootState) => state.breadCrumb.tid;
export const selectDivision = (state: RootState) => state.breadCrumb.division;
export const selectDid = (state: RootState) => state.breadCrumb.did;
export const selectRoom = (state: RootState) => state.breadCrumb.room;
export const selectRound = (state: RootState) => state.breadCrumb.round;
export const selectTeam = (state: RootState) => state.breadCrumb.team;
