import { For, Show, createResource } from "solid-js";
import ExpenseComponent, { Expense } from "../common/Expense";
import Balance from "../common/Balance";
import { AddAndSettleButtons } from "../common/HomeButtons";

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
                    <AddAndSettleButtons/>
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

