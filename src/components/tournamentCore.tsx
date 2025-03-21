import { useShallow } from "zustand/shallow";
import useTournamentsStore from "../stores/tournament";
import { useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

function TournamentCore() {
    const [currentTournament, setTournamentActive] = useTournamentsStore(useShallow((state) => [state.selected_id, state.set_active]))

    useEffect(() => {
        if (currentTournament) {
            loadUsers()
        }
    }, [currentTournament])

    const loadUsers = async () => {
        await invoke("load_users", {tournamentId: currentTournament})
            .then(() => setTournamentActive(true))
    }

    return <>
    </>
}

export default TournamentCore;