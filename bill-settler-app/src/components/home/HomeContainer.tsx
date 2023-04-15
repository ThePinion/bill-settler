import AddExpenseButton from "../common/AddExpenseButton";
import ExpenseComponent, { Expense } from "../common/Expense";
import Gap from "../common/Gap";
import SettleUpButton from "../common/SettleUpButton";
import Balance from "./Balance";

export default function HomeContainer() {
    return(
        <div class="content-container">
            <div class="flex flex-wrap flex-row items-center justify-center mt-10">
                <Balance owe={17.34} owed={24.16} />
                <Gap/>
                <Gap/>
                <AddExpenseButton/>
                <Gap/>
                <SettleUpButton/>
            </div>
            <h1 class="text-3xl font-bold my-12">Recent Expenses</h1>
            <ExpenseComponent expense={new Expense("name1", "owner1", 15)}/>
            <ExpenseComponent expense={new Expense("name2", "owner2", 123.11)}/>
            <ExpenseComponent expense={new Expense("name3", "owner1", .12)}/>
            <ExpenseComponent expense={new Expense("name4", "owner4", 10.01)}/>
        </div>
    );
}