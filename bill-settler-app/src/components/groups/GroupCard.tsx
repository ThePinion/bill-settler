import { A } from "@solidjs/router";
import { Component } from "solid-js";

export class Group {
    public id: number;
    public name: string;

    constructor(id: number, name: string) {
        this.id = id;
        this.name = name;
    }

}

const GroupCard: Component<{group: Group}> = (props) => {
    return(
        <A
            href={"/group/" + props.group.id}
            class="flex items-center justify-center rounded-xl px-12 py-8 shadow-md
            bg-gray-700 hover:bg-[#253041] transition-all duration-200 hover:rounded-lg
            text-slate-100 hover:text-slate-300 h-24"
        >
            <div class="text-lg font-semibold">
                {props.group.name}
            </div>
        </A>
    );
}

export default GroupCard