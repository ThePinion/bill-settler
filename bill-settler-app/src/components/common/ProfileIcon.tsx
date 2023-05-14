import { Component } from "solid-js";
import { FaRegularUser } from "solid-icons/fa";
import { TbLetterA, TbLetterB, TbLetterC, TbLetterD, TbLetterE, TbLetterF, TbLetterG, TbLetterH, TbLetterI, TbLetterJ, TbLetterK, TbLetterL, TbLetterM, TbLetterN, TbLetterO, TbLetterP, TbLetterQ, TbLetterR, TbLetterS, TbLetterT, TbLetterU, TbLetterV, TbLetterW, TbLetterX, TbLetterY, TbLetterZ } from "solid-icons/tb";

const ProfileIcon : Component<{profileName: string}> = (props) => {
    switch (props.profileName.at(0)?.toLowerCase()) {
        case "a":
            return(
                <TbLetterA class="rounded-3xl" size={24}/>
            );
        case "b":
            return(
                <TbLetterB class="rounded-3xl" size={24}/>
            );
        case "c":
            return(
                <TbLetterC class="rounded-3xl" size={24}/>
            );
        case "d":
            return(
                <TbLetterD class="rounded-3xl" size={24}/>
            );
        case "e":
            return(
                <TbLetterE class="rounded-3xl" size={24}/>
            );
        case "f":
            return(
                <TbLetterF class="rounded-3xl" size={24}/>
            );
        case "g":
            return(
                <TbLetterG class="rounded-3xl" size={24}/>
            );
        case "h":
            return(
                <TbLetterH class="rounded-3xl" size={24}/>
            );
        case "i":
            return(
                <TbLetterI class="rounded-3xl" size={24}/>
            );
        case "j":
            return(
                <TbLetterJ class="rounded-3xl" size={24}/>
            );
        case "k":
            return(
                <TbLetterK class="rounded-3xl" size={24}/>
            );
        case "l":
            return(
                <TbLetterL class="rounded-3xl" size={24}/>
            );
        case "m":
            return(
                <TbLetterM class="rounded-3xl" size={24}/>
            );
        case "n":
            return(
                <TbLetterN class="rounded-3xl" size={24}/>
            );
        case "o":
            return(
                <TbLetterO class="rounded-3xl" size={24}/>
            );
        case "p":
            return(
                <TbLetterP class="rounded-3xl" size={24}/>
            );
        case "q":
            return(
                <TbLetterQ class="rounded-3xl" size={24}/>
            );
        case "r":
            return(
                <TbLetterR class="rounded-3xl" size={24}/>
            );
        case "s":
            return(
                <TbLetterS class="rounded-3xl" size={24}/>
            );
        case "t":
            return(
                <TbLetterT class="rounded-3xl" size={24}/>
            );
        case "u":
            return(
                <TbLetterU class="rounded-3xl" size={24}/>
            );
        case "v":
            return(
                <TbLetterV class="rounded-3xl" size={24}/>
            );
        case "w":
            return(
                <TbLetterW class="rounded-3xl" size={24}/>
            );
        case "x":
            return(
                <TbLetterX class="rounded-3xl" size={24}/>
            );
        case "y":
            return(
                <TbLetterY class="rounded-3xl" size={24}/>
            );
        case "z":
            return(
                <TbLetterZ class="rounded-3xl" size={24}/>
            );
        default:
            return(
                <FaRegularUser class="rounded-3xl" size={24}/>
            );
    }
}


export default ProfileIcon;