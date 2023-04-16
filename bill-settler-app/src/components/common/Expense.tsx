import { Component } from "solid-js";
import Gap from "./Gap";

export class Expense {
    public name: string;
    public owner: string;
    public cost: number;

    constructor(name: string, owner: string, cost: number) {
        this.name = name;
        this.owner = owner;
        this.cost = cost;
    }

}

const ExpenseComponent: Component<{expense: Expense}> = (props) => {
    return(
        <button 
            class="flex flex-row flex-wrap rounded-xl p-4 m-2 w-96 shadow-sm
            bg-gray-700 hover:bg-[#253041] transition-all duration-200 hover:rounded-lg"
        >
            <div class="font-semibold">{props.expense.name}</div>
            <Gap/>
            <div class="font-light">{"paid by " + props.expense.owner}</div>
            <Gap/>
            <div class="w-24 text-right">{props.expense.cost + "$"}</div>
        </button>
    );
}

export default ExpenseComponent