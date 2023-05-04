import { For, Show, createResource } from "solid-js";
import AddExpenseButton from "../common/AddExpenseButton";
import ExpenseComponent, { Expense } from "../common/Expense";
import SettleUpButton from "../common/SettleUpButton";
import Balance from "./Balance";

const fetchRecentExpenses = async () => {
    const res = await fetch('http://localhost:4000/recent-expenses')

    return res.json()
}

const fetchBalance = async () => {
    const res = await fetch('http://localhost:4000/balance')

    return res.json()
}

export default function HomeContainer() {

    const [recentExpenses] = createResource(fetchRecentExpenses)
    const [balance] = createResource(fetchBalance)

    return(
        <div class="content-container">
            <Show when={balance() && recentExpenses()}>
                <div class="flex flex-wrap flex-row items-center justify-center mt-10 gap-24">
                    <Balance owe={balance().owe} owed={balance().owed} />
                    <HomeButtons/>
                </div>
                <h1 class="text-3xl font-bold my-12">Recent Expenses</h1>
                <For each={recentExpenses()}>
                    {(expense) => (
                        <ExpenseComponent expense={expense}/>
                    )}
                </For>
            </Show>
        </div>
    );
}

function HomeButtons() {
    return(
        <div class="flex flex-row gap-8">
            <AddExpenseButton/>
            <SettleUpButton/>
        </div>
    );
}