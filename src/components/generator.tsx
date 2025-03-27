import { Button } from "antd";
import useTournamentsStore from "../stores/tournament";
import { invoke } from "@tauri-apps/api/core";

function Generator() {
    
    const currentTournament = useTournamentsStore((state) => state.selected_id);

    async function startGeneration() {
        await invoke("invoke_generation", {tournamentId: currentTournament})
    }

    return <div style={{paddingTop: 15}}>
        <Button onClick={() => startGeneration()}>Generate stats</Button>
    </div>
}

export default Generator;