import { A } from "@solidjs/router";
import { For, Index } from "solid-js";
import { Profile } from "../../model/Profile";
import { createStore, produce } from "solid-js/store";


export default function NewExpense() {

    const [paidFor, setPaidFor] = createStore(new Array<Profile>());
    let paidForInput: HTMLInputElement;

    const addUser = (username: string) => {
        setPaidFor(
            produce((paidFor) => {
                paidFor.push(new Profile(username))
            })
        )
    };

    const deleteUser = (index: number) => {
        setPaidFor(
            produce((paidFor) => {
                paidFor.splice(index, 1)
            })
        )
    }

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
                <label class="block font-bold mb-2 " for="expense_name">Name</label>
                <input
                    class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                    type="text"
                    id="expense_name"
                    name="expense_name"
                    autocomplete="off"
                />
                <label class="block font-bold my-2" for="paid_by">Paid by</label>
                <input
                    class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                    type="text"
                    id="paid_by"
                    name="paid_by"
                    autocomplete="off"
                />
                <label class="block font-bold my-2" for="cost">Cost</label>
                <input
                    class="block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                    type="number"
                    id="cost"
                    name="cost"
                    step={.01}
                    autocomplete="off"
                />
                <label class="block font-bold my-2" for="paid_for">Paid for</label>
                <div class="flex flex-row gap-3">
                    <input
                        class=" flex-auto block w-full px-3 py-2 rounded-lg bg-gray-700 shadow-md border border-gray-700 focus:outline-none focus:border-blue-500"
                        type="text"
                        id="paid_for"
                        name="paid_for"
                        ref={paidForInput}
                        placeholder="Enter username"
                        autocomplete="off"
                    />
                    <button
                        class="text-center px-4 rounded-lg bg-gray-700 shadow-md
                        hover:rounded-mg hover:bg-[#2b384b]"
                        onClick={() => {
                            if(!paidForInput.value.trim()) return;
                            addUser(paidForInput.value.trim());
                            paidForInput.value = ""
                        }}
                    >
                        Add
                    </button>
                </div>
                <div
                    class="w-full h-40 my-2 overflow-x-hidden overflow-y-auto
                    scrollbar-thin scrollbar-thumb-gray-800" 
                >
                    <Index each={paidFor}>
                        {(profile, index) => (
                            <div
                                class="my-4 mx-1 py-2 px-4 rounded-2xl bg-gray-700
                                flex flex-row shadow-md"
                            >
                                <div class="flex-auto py-1">
                                    {profile()  .name}
                                </div>
                                <button
                                    class="bg-[#2c394d] rounded-3xl px-2 py-1
                                    hover:rounded-xl hover:bg-[#2b384b] shadow-sm"
                                    onClick={[deleteUser, index]}
                                >
                                    Delete
                                </button>
                            </div>
                        )}
                    </Index>
                </div>
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
            </div>
            
        </div>
    )
}