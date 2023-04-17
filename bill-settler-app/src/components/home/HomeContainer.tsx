import AddExpenseButton from "../common/AddExpenseButton";
import ExpenseComponent, { Expense } from "../common/Expense";
import Gap from "../common/Gap";
import SettleUpButton from "../common/SettleUpButton";
import Balance from "./Balance";

export default function HomeContainer() {
    return(
        <div class="content-container">
            <div class="flex flex-wrap flex-row items-center justify-center mt-10 gap-24">
                <Balance owe={17.34} owed={24.16} />
                <HomeButtons/>
            </div>
            <h1 class="text-3xl font-bold my-12">Recent Expenses</h1>
            <ExpenseComponent expense={new Expense("name1", "owner1", 15, true)}/>
            <ExpenseComponent expense={new Expense("name2", "owner2", 123.11, true)}/>
            <ExpenseComponent expense={new Expense("name3", "owner1", .12, false)}/>
            <ExpenseComponent expense={new Expense("name4", "owner4", 10.01, true)}/>
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