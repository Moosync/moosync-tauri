// Moosync
// Copyright (C) 2024, 2025  Moosync <support@moosync.app>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use leptos::{component, prelude::*, view, IntoView};

#[tracing::instrument(level = "debug", skip())]
#[component]
pub fn ImportPlaylistIcon() -> impl IntoView {
    view! {
        <svg viewBox="0 0 72 72" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M31.8 40.1998C32.2333 40.7775 32.4437 41.4921 32.3925 42.2125C32.3413 42.9328 32.032 43.6105 31.5214 44.1211C31.0107 44.6317 30.333 44.9411 29.6127 44.9922C28.8924 45.0434 28.1778 44.8331 27.6 44.3998C26.1651 43.0531 25.0214 41.4266 24.2396 39.6206C23.4578 37.8147 23.0545 35.8677 23.0545 33.8998C23.0545 31.9319 23.4578 29.9849 24.2396 28.179C25.0214 26.373 26.1651 24.7465 27.6 23.3998L38.1 12.5998C40.9545 9.83231 44.7742 8.28467 48.75 8.28467C52.7259 8.28467 56.5455 9.83231 59.4 12.5998C62.1675 15.4543 63.7152 19.274 63.7152 23.2498C63.7152 27.2256 62.1675 31.0453 59.4 33.8998L54.9 38.3998C54.9641 35.9451 54.5568 33.501 53.7 31.1998L55.2 29.6998C56.8022 27.9336 57.6897 25.6344 57.6897 23.2498C57.6897 20.8652 56.8022 18.566 55.2 16.7998C53.4339 15.1976 51.1346 14.3102 48.75 14.3102C46.3655 14.3102 44.0662 15.1976 42.3 16.7998L31.8 27.5998C30.9468 28.4124 30.2676 29.3897 29.8035 30.4727C29.3394 31.5556 29.1 32.7216 29.1 33.8998C29.1 35.078 29.3394 36.2439 29.8035 37.3269C30.2676 38.4098 30.9468 39.3872 31.8 40.1998ZM69 53.9998V59.9998H60V68.9998H54V59.9998H45V53.9998H54V44.9998H60V53.9998M48.6 41.0998C49.12 38.6783 49.0066 36.1637 48.2709 33.7989C47.5352 31.434 46.2019 29.2989 44.4 27.5998C43.8223 27.1665 43.1077 26.9561 42.3874 27.0073C41.667 27.0585 40.9893 27.3678 40.4787 27.8785C39.9681 28.3891 39.6588 29.0668 39.6076 29.7871C39.5564 30.5075 39.7667 31.2221 40.2 31.7998C41.0532 32.6124 41.7325 33.5897 42.1966 34.6727C42.6607 35.7556 42.9 36.9216 42.9 38.0998C42.9 39.278 42.6607 40.4439 42.1966 41.5269C41.7325 42.6098 41.0532 43.5872 40.2 44.3998L29.7 55.1998C27.9339 56.8019 25.6346 57.6894 23.25 57.6894C20.8655 57.6894 18.5662 56.8019 16.8 55.1998C15.1979 53.4336 14.3104 51.1344 14.3104 48.7498C14.3104 46.3652 15.1979 44.066 16.8 42.2998L18.3 41.0998C17.4663 38.6888 17.0601 36.1506 17.1 33.5998L12.6 38.0998C9.83255 40.9543 8.28491 44.774 8.28491 48.7498C8.28491 52.7256 9.83255 56.5453 12.6 59.3998C15.4545 62.1673 19.2742 63.7149 23.25 63.7149C27.2259 63.7149 31.0455 62.1673 33.9 59.3998L39.3 53.9998C39.7586 51.2809 40.8357 48.7037 42.4482 46.467C44.0606 44.2304 46.1654 42.3941 48.6 41.0998Z"
                fill="var(--textPrimary)"
            ></path>
        </svg>
    }
}
