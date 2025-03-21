import InfiniteScroll from "react-infinite-scroll-component";
import useTournamentsStore from "../stores/tournament";
import { useEffect, useState } from "react";
import { Match } from "../types/tournament";
import { invoke } from "@tauri-apps/api/core";
import { List, Typography } from "antd";
import { useShallow } from "zustand/shallow";

function MatchesList() {
    const [tournamentIsActive, currentTournament] = useTournamentsStore(useShallow((state) => [state.active, state.selected_id]));
    const [matches, setMatches] = useState<Match[]>([]);

    useEffect(() => {
        if (tournamentIsActive) {
            console.log("current tournament: ", currentTournament)
            loadMatches()
        }
    }, [tournamentIsActive])

    const loadMatches = async () => {
        await invoke<Match[]>("load_matches", {tournamentId: currentTournament})
            .then((values) => {
                console.log("matches: ", values)
                setMatches(values)
            })
    }

    return <>
        <InfiniteScroll
            dataLength={matches.length}
            height={500}
            hasMore={false}
            next={() => {}}
            loader={<h4>Loading...</h4>}
        >
            <List>{matches.map((m, i) => (
                <List.Item key={i}>
                    <div style={{display: 'flex', flexDirection: 'row', gap: 5}}>
                        <Typography.Text>{`${m.first_user_nickname} VS ${m.second_user_nickname}`}</Typography.Text>
                    </div>
                </List.Item>
            ))}</List>
        </InfiniteScroll>
    </>
}

export default MatchesList;