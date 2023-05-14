import { A } from "@solidjs/router";


export default function NewExpense() {
    return(
        <div 
            class="bg-gray-800 text-white h-screen flex justify-center items-center"
        >
            <div
                class="p-8 rounded-lg shadow-xl bg-[#253041]"
            >
                <h1 class="font-bold text-2xl mb-6 flex justify-center">
                    New Expense
                </h1>
                <form>
                    <label class="block font-bold mb-2 " for="expense_name">Name</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="expense_name"
                        name="expense_name"
                    />
                    <label class="block font-bold my-2" for="paid_by">Paid by</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="paid_by"
                        name="paid_by"
                    />
                    <label class="block font-bold my-2" for="cost">Cost</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="number"
                        id="cost"
                        name="cost"
                    />
                    <label class="block font-bold my-2" for="paid_for">Paid for</label>
                    <input
                        class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="paid_for"
                        name="paid_for"
                    />
                    <div class="flex flex-row gap-8 items-center justify-center mt-8">
                        <A href="/home">
                            <button class="main-button bg-teal-600 hover:bg-teal-700">
                                Add Expense
                            </button>
                        </A>
                        <A href="/home">
                            <button class="main-button bg-yellow-600 hover:bg-yellow-700">
                                Cancel
                            </button>
                        </A>
                    </div>
                </form>
            </div>
            
        </div>
    )
}