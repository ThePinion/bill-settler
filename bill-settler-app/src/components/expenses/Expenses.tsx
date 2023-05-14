import SideBar from "../menu/SideBar";
import ExpensesContainer from "./ExpensesContainer";

export default function Expenses() {
    return(
        <div class="flex flex-row h-screen">
            <SideBar/>
            <ExpensesContainer/>
        </div>
    );
}