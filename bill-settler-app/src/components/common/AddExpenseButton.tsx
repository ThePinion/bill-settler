import { A } from "@solidjs/router"

export default function AddExpenseButton() {
    return(
        <A href="/new-expense">
            <button class="main-button bg-teal-600 hover:bg-teal-700">
                Add an Expense
            </button>
        </A>
    );
}