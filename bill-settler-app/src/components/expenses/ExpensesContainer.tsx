import { For, Show, createResource } from "solid-js";
import Balance from "../common/Balance";
import ExpenseComponent from "../common/Expense";
import { AddAndSettleButtons } from "../common/HomeButtons";

const fetchExpenses = async () => {
    const res = await fetch('http://localhost:4000/expenses')

    return res.json()
}

const fetchBalance = async () => {
    const res = await fetch('http://localhost:4000/balance')

    return res.json()
}

export default function ExpensesContainer() {

    const [expenses] = createResource(fetchExpenses)
    const [balance] = createResource(fetchBalance)

    return(
        <div class="content-container">
            <Show when={balance() && expenses()}>
                <div class="flex flex-wrap flex-row items-center justify-center mt-10 gap-24">
                    <Balance owe={balance().owe} owed={balance().owed} />
                    <AddAndSettleButtons/>
                </div>
                <h1 class="text-3xl font-bold my-12">Expenses</h1>
                <For each={expenses()}>
                    {(expense) => (
                        <ExpenseComponent expense={expense}/>
                    )}
                </For>
            </Show>
        </div>
    );
}