import { Component, JSX } from "solid-js";
import { TbLetterU } from 'solid-icons/tb'
import { A } from "@solidjs/router";
import { IconTypes } from "solid-icons";

export class Profile {
    name: string;

    constructor(name: string) {
        this.name = name;
    }
}

const ProfileComponent: Component<{profile: Profile}> = (props) => {
    return(
        <A
            href="/profile"
            class="flex flex-row relative items-start mx-2 my-6 p-2
            text-xl font-bold hover:bg-gray-800 hover:text-teal-700
            transition-all duration-100 rounded-3xl"
        >
            <div class="aspect-square h-full bg-sky-950 text-teal-700 rounded-3xl p-2">
                <TbLetterU class="rounded-3xl" size={24}/>
            </div>
            
            <div class="h-full p-1 ml-2">
                {props.profile.name}
            </div>
        </A>
    );
}

function getIcon(profile: Profile) : JSX.Element {
    
    return(
        <TbLetterU class="rounded-3xl" size={24}/>
    );
}

export default ProfileComponent