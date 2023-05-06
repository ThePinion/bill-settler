import { For, Show, createResource } from "solid-js";
import GroupCard from "./GroupCard";

const fetchGroups = async () => {
    const res = await fetch('http://localhost:4000/groups')

    return res.json()
}

export default function GroupsContainer() {

    const [groups] = createResource(fetchGroups)

    return(
        <div class="content-container">
            <h1 class="text-3xl font-bold my-12">Your Groups</h1>
            <Show when={groups()}>
                <div class="grid grid-cols-4 gap-10">
                    <For each={groups()}>
                        {(group) => (
                            <GroupCard group={group}></GroupCard>
                        )}
                    </For>
                </div>
            </Show>
        </div>
    );
}