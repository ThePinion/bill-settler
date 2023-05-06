import AddExpenseButton from "./AddExpenseButton";
import SettleUpButton from "./SettleUpButton";

export function AddAndSettleButtons() {
    return (
        <div class="flex flex-row gap-8">
            <AddExpenseButton />
            <SettleUpButton />
        </div>
    );
}
