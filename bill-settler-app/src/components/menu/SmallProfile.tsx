import { Component, JSX } from "solid-js";
import { A } from "@solidjs/router";
import ProfileIcon from "../common/ProfileIcon";

export class Profile {
    name: string;

    constructor(name: string) {
        this.name = name;
    }
}

const SmallProfileComponent: Component<{profile: Profile}> = (props) => {
    return(
        <A
            href="/profile"
            class="flex flex-row relative items-start mx-2 my-6 p-2
            text-xl font-bold hover:bg-gray-800 hover:text-teal-700
            transition-all duration-100 rounded-3xl"
        >
            <div class="aspect-square h-full bg-sky-950 text-teal-700 rounded-3xl p-2">
                <ProfileIcon profileName={props.profile.name}></ProfileIcon>
            </div>
            
            <div class="h-full p-1 ml-2">
                {props.profile.name}
            </div>
        </A>
    );
}

export default SmallProfileComponent