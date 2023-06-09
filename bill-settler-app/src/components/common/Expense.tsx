import { Component } from "solid-js";
import { Expense } from "../../model/Expense";
import { A } from "@solidjs/router";

const ExpenseComponent: Component<{expense: Expense}> = (props) => {

    let settledString = props.expense.settled ? "Settled" : "Unsettled"

    return(
        <A
            href={"/expense/" + props.expense.id}
            class="flex flex-row flex-wrap items-center justify-center rounded-xl px-4 py-2 m-2 shadow-sm
            bg-gray-700 hover:bg-[#253041] transition-all duration-200 hover:rounded-lg"
        >
            <div class="flex flex-col text-left w-52 overflow-hidden">
                <div class="font-semibold text-lg">{props.expense.name}</div>
                <div class="font-extralight text-sm">Paid by {props.expense.owner}</div>
            </div>
            <div class="w-16 text-right font-mono font-semibold text-green-500 text-opacity-80">
                {props.expense.cost.toFixed(2)}$
            </div>
            <div class="w-16 ml-6 text-right text-gray-400">
                {settledString}
            </div>
        </A>
    );
}

export default ExpenseComponent