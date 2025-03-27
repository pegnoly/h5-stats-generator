import InfiniteScroll from "react-infinite-scroll-component";
import useTournamentsStore from "../stores/tournament";
import { useEffect, useState } from "react";
import { Match } from "../types/tournament";
import { invoke } from "@tauri-apps/api/core";
import { List, Typography } from "antd";
import { useShallow } from "zustand/shallow";
import { Link } from "react-router";

function MatchesList() {
    const [tournamentIsActive, currentTournament, focusedMatch] = useTournamentsStore(useShallow((state) => [state.active, state.selected_id, state.focused_match]));
    const [matches, setMatches] = useState<Match[]>([]);

    useEffect(() => {
        if (tournamentIsActive) {
            //console.log("current tournament: ", currentTournament)
            loadMatches()
        }
    }, [tournamentIsActive])

    const loadMatches = async () => {
        await invoke<Match[]>("load_matches", {tournamentId: currentTournament})
            .then((values) => {
                //console.log("matches: ", values)
                setMatches(values)
            })
    }

    return <>
        <InfiniteScroll
            dataLength={matches.length}
            height={550}
            hasMore={false}
            next={() => {}}
            loader={<h4>Loading...</h4>}
        >
            <List>{matches.map((m, i) => (
                <List.Item key={i} style={{ backgroundColor: m.id == focusedMatch ? 'green' : 'white'}}>
                    <Link to={`focus_match/${m.id}`}>
                        <div style={{display: 'flex', flexDirection: 'row', gap: 5}}>
                            <Typography.Text>{`${m.first_user_nickname} VS ${m.second_user_nickname}`}</Typography.Text>
                        </div>
                    </Link>
                </List.Item>
            ))}</List>
        </InfiniteScroll>
    </>
}

export default MatchesList;