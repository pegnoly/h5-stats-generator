import { invoke } from "@tauri-apps/api/core";
import { Button, Select } from "antd";
import { useEffect, useState } from "react";
import { Tournament } from "./types/tournament";
import useTournamentsStore from "./stores/tournament";
import MatchesList from "./components/matchesList";
import TournamentCore from "./components/tournamentCore";
import { useShallow } from "zustand/shallow";
import { Route, Routes } from "react-router";
import MatchFocused from "./components/matchFocused";
import Generator from "./components/generator";

type TournamentData = {
    id: string,
    name: string
}

function App() {

    const [tournaments, setTournaments] = useState<TournamentData[]>([]);
    const [setCurrentTournament, setTournamentActive] = useTournamentsStore(useShallow((state) => [state.set_id, state.set_active]))

    useEffect(() => {
        invoke<TournamentData[]>("load_tournaments_list")
            .then((value) => setTournaments(value))
    }, [])

    async function selectTournament(selectedId: string) {
        setTournamentActive(false);
        await invoke<Tournament>("load_tournament", {tournamentId: selectedId})
            .then((value) => {
                invoke("load_heroes", {modType: value.mod_type});
                setCurrentTournament(value.id)
            })
    }

    return (
        <>
            <Select
                onChange={selectTournament}
            >{tournaments.map((t, i) => (
                <Select.Option key={i} value={t.id}>{t.name}</Select.Option>
            ))}</Select>
            <TournamentCore/>
            <div style={{height: '80%', width: '100%', display: 'flex', flexDirection: 'row'}}>
                <div style={{height: '100%', width: '35%'}}>
                    <MatchesList/>
                </div>
                <Routes>
                    <Route
                        path="focus_match/:id"
                        element={
                            <MatchFocused/>
                        }
                    />
                </Routes>
            </div>
            <Generator/>
        </>
    )
}

export default App;