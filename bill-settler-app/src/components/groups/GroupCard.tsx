import { Component } from "solid-js";

export class Group {
    public name: string;

    constructor(name: string) {
        this.name = name;
    }

}

const GroupCard: Component<{group: Group}> = (props) => {
    return(
        <div
            class="flex items-center justify-center rounded-xl px-12 py-8 shadow-md
            bg-gray-700 hover:bg-[#253041] transition-all duration-200 hover:rounded-lg
            text-slate-100 hover:text-slate-300 h-24"
        >
            <div class="text-lg font-semibold">
                {props.group.name}
            </div>
        </div>
    );
}

export default GroupCard