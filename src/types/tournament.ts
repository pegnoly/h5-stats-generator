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