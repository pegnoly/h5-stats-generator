import { create } from "zustand"

type State = {
    selected_id: string | null,
    active: boolean
}

type Action = {
    set_id: (id: string) => void,
    set_active: (is_active: boolean) => void
}

const useTournamentsStore = create<State & Action>((set) => ({
    selected_id: null,
    active: false,
    set_id(id) {
        set({selected_id: id})
    },
    set_active(is_active) {
        set({active: is_active})
    },
}))

export default useTournamentsStore;