import { invoke } from "@tauri-apps/api/core";
import { Segmented, Select } from "antd";
import { useEffect, useState } from "react";
import { useParams } from "react-router";
import { BargainsColor, Game, GameOutcome, GameResult, Hero, racesData } from "../types/tournament";
import useTournamentsStore from "../stores/tournament";

function MatchFocused() {

    const {id} = useParams();
    const [gamesIds, setGamesIds] = useState<string[]>([]);
    const [currentGameId, setCurrentGameId] = useState<string | null>(null);
    const setFocusedMatch = useTournamentsStore((state) => state.set_focused);

    useEffect(() => {
        if (id != undefined) {
            setFocusedMatch(id)
            invoke<string[]>("load_games", {matchId: id})
                .then((ids) => {
                    setGamesIds(ids);
                    setCurrentGameId(ids[0])
                })
        }
    }, [id])

    return <div style={{width: '65%', height: '100%'}}>
        {
            !currentGameId ? 
            null :
            <div style={{display: 'flex', flexDirection: 'column'}}>
                <Segmented 
                    options={BuildGameList(gamesIds)}
                    value={currentGameId}
                    onChange={(value) => setCurrentGameId(value)}
                /> 
                <GameRenderer gameId={currentGameId}/>
            </div> 
        }
    </div>
}

function BuildGameList(ids: string[]) {
    let count = 0;
    const data = ids.map((id) => {
        count++;
        return {value: id, label: `Игра ${count}`}
    })
    return data;
}

function GameRenderer({gameId} : {gameId: string}) {

    const [firstPlayerRace, setFirstPlayerRace] = useState<number>(-1);
    const [firstPlayerHero, setFirstPlayerHero] = useState<number>(-1);
    const [secondPlayerRace, setSecondPlayerRace] = useState<number>(-1);
    const [secondPlayerHero, setSecondPlayerHero] = useState<number>(-1);
    const [bargainsColor, setBargainsColor] = useState<BargainsColor | null>(null);
    const [bargainsAmount, setBargainsAmount] = useState<number>(-1);
    const [result, setResult] = useState<GameResult>(GameResult.NotSelected);
    const [outcome, setOutcome] = useState<GameOutcome>(GameOutcome.FinalBattleVictory);

    useEffect(() => {
        loadGame()
    }, [gameId])

    async function updateFirstPlayerHero(newHero: number) {
        await invoke("update_game_first_player_hero", {gameId: gameId, hero: newHero});
        setFirstPlayerHero(newHero);
    }

    const loadGame = async () => {
        await invoke<Game>("select_game", {gameId: gameId})
            .then((game) => {
                console.log("Game: ", game);
                setFirstPlayerRace(game.first_player_race);
                setFirstPlayerHero(game.first_player_hero);
                setSecondPlayerRace(game.second_player_race);
                setSecondPlayerHero(game.second_player_hero);
                setBargainsColor(game.bargains_color);
                setBargainsAmount(game.bargains_amount);
                setResult(game.result);
                setOutcome(game.outcome);
            })
    }

    return <>{
        gameId == undefined ? 
        null : 
        <div style={{width: '100%', height: '65%', display: 'flex', flexDirection: 'column', gap: 10}}>
            <h4>{gameId}</h4>
            <div style={{width: '100%', display: 'flex', flexDirection: 'row', gap: 5, paddingTop: 25, paddingLeft: 7}}>
                <PlayerDataRenderer race={firstPlayerRace} hero={firstPlayerHero} updateRace={setFirstPlayerRace} updateHero={updateFirstPlayerHero}/>
                <ResultDataRenderer result={result} updateResult={setResult}/>
                <PlayerDataRenderer race={secondPlayerRace} hero={secondPlayerHero} updateRace={setSecondPlayerRace} updateHero={setSecondPlayerHero}/>
            </div>
        </div>
    }</>
}

function PlayerDataRenderer({race, hero, updateRace, updateHero}: {
    race: number, 
    hero: number,
    updateRace: (r: number) => void,
    updateHero: (h: number) => void
}) {
    const [heroesData, setHeroesData] = useState<Hero[]>([]);
    
    useEffect(() => {
        console.log(race)
        if (race != undefined) {
            getHeroes()
        }
    }, [race])

    const getHeroes = async () => {
        await invoke<Hero[]>("get_heroes_of_race", {race: race})
            .then((heroes) => {
                setHeroesData(heroes);
            })
    }

    return <div style={{display: 'flex', flexDirection: 'column', width: '32%'}}>
        <Select
            onChange={updateRace}
            value={race}
        >{Array.from(racesData.entries()).map((v, i) => (
            <Select.Option key={i} value={v[0]}>{v[1]}</Select.Option>
        ))}</Select>
        <Select
            onChange={updateHero}
            value={hero}
        >{heroesData.map((h, i) => (
            <Select.Option key={i} value={h.id}>{h.name}</Select.Option>
        ))}</Select>
    </div>
}

function ResultDataRenderer({result, updateResult}: {
    result: GameResult,
    updateResult: (r: GameResult) => void
}) {

    async function updateResultData(newResult: GameResult) {
        updateResult(newResult)
    }

    return <div style={{display: 'flex', flexDirection: 'column', width: "32%"}}>
        <Select
            value={result}
            onChange={updateResultData}
        >
            <Select.Option key={0} value={GameResult.NotSelected}>Не определено</Select.Option>
            <Select.Option key={1} value={GameResult.FirstPlayerWon}>Победил</Select.Option>
            <Select.Option key={2} value={GameResult.SecondPlayerWon}>Проиграл</Select.Option>
        </Select>
    </div>
}

export default MatchFocused;