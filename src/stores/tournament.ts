import { create } from "zustand"

type State = {
    selected_id: string | null,
    active: boolean,
    focused_match: string | null
}

type Action = {
    set_id: (id: string) => void,
    set_active: (is_active: boolean) => void,
    set_focused: (focused: string) => void
}

const useTournamentsStore = create<State & Action>((set) => ({
    selected_id: null,
    active: false,
    focused_match: null,
    set_id(id) {
        set({selected_id: id})
    },
    set_active(is_active) {
        set({active: is_active})
    },
    set_focused(focused) {
        set({focused_match: focused})
    },
}))

export default useTournamentsStore;