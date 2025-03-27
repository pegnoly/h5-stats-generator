export enum ModType {
    Universe,
    Hrta
}

export enum GameType {
    Rmg,
    Arena
}

export type Tournament = {
    id: string,
    name: string,
    mod_type: ModType,
    game_type: GameType, 
    with_bargains: boolean,
    with_bargains_color: boolean,
    with_foreign_heroes: boolean
}

export type Match = {
    id: string,
    first_user_id: string,
    first_user_nickname: string,
    second_user_id: string,
    second_user_nickname: string
}

export enum BargainsColor {
    NotSelected,
    BargainsColorRed,
    BargainsColorBlue
}

export enum GameResult {
    NotSelected = "NotSelected",
    FirstPlayerWon = "FirstPlayerWon",
    SecondPlayerWon = "SecondPlayerWon"
}

export enum GameOutcome {
    FinalBattleVictory,
    NeutralsVictory,
    OpponentSurrender
}

export type Game = {
    id: string,
    first_player_race: number,
    first_player_hero: number,
    second_player_race: number,
    second_player_hero: number,
    bargains_color: BargainsColor | null,
    bargains_amount: number,
    result: GameResult,
    outcome: GameOutcome
}

export type Hero = {
    id: number,
    name: number
}

export const racesData = new Map<number, string>([
    [1, "Орден порядка"],
    [2, "Инферно"],
    [3, "Некрополис"],
    [4, "Лесной союз"],
    [5, "Лига теней"],
    [6, "Академия волшебства"],
    [7, "Северные кланы"],
    [8, "Великая орда"]
])