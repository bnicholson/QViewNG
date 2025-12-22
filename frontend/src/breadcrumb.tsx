
import { createSlice, type PayloadAction } from '@reduxjs/toolkit'

interface BreadCrumbState {
    displayDate: number,
    isOn: boolean,
    tournament: String,
    tid: number;
    division: String,
    did: number;
    room: String,
    round: String,
    team: String,
}

const initialState : BreadCrumbState = {
    displayDate: Date.now(),
    isOn: true,
    tournament: "",
    tid: -1,
    division: "",
    did: -1,
    room: "",
    round: "",
    team: "",
}

export const breadCrumb = createSlice({
    name: 'breadcrumb',
    initialState,
    reducers: {
        toggleIsOn: (state) => {
            state.isOn = !state.isOn
        },
        setTournament: (state, action : PayloadAction<String>) => {
            state.tournament = action.payload
        },
        setTid: (state, action : PayloadAction<number>) => {
            state.tid = action.payload 
        },
        setDivision: (state, action: PayloadAction<String>) => {
            state.division = action.payload
        },
        setDid: (state, action: PayloadAction<number>) => {
            state.did = action.payload
        },
        setDisplayDate: (state, action: PayloadAction<number>) => {
            state.displayDate = action.payload
        },
        setRoom: (state, action: PayloadAction<String>) => {
            state.room = action.payload
        },
        setRound: (state, action: PayloadAction<String>) => {
            state.round = action.payload
        },
        setTeam: (state, action: PayloadAction<String>) => {
            state.team = action.payload
        },
    },

})

export default breadCrumb.reducer;

export const { setDisplayDate, toggleIsOn, setTournament, setTid, setDivision, setDid, setRoom, setRound, setTeam } = breadCrumb.actions;
