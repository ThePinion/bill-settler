import { Component, Show, createResource } from "solid-js";

const fetchGroup = async (id: number) => {
    const res = await fetch('http://localhost:4000/groups/' + id)

    return res.json()
}

const GroupContainer: Component<{id: number}> = (props) => {

    const [group] = createResource(props.id, fetchGroup)

    return(
        <div class="content-container">
            <Show when={group()}>
                <div class="mx-auto text-3xl font-bold">
                    {group().name}
                    
                </div>
            </Show>
        </div>
    );
}

export default GroupContainer