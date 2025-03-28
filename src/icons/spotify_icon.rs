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

#[tracing::instrument(level = "debug", skip(fill))]
#[component]
pub fn SpotifyIcon(#[prop(optional)] fill: String) -> impl IntoView {
    view! {
        <svg viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
            <title>Spotify</title>
            <g>
                <rect
                    x="3"
                    y="3"
                    width="12"
                    height="10"
                    fill=if fill.is_empty() { "white".into() } else { fill.clone() }
                />
                <path
                    d="M8.5 0C3.80552 0 0 3.80552 0 8.5C0 13.1945 3.80552 17 8.5 17C13.1945 17 17 13.1945 17 8.5C17 3.80552 13.1945 0 8.5 0ZM11.1244 11.8973C10.9933 11.8973 10.903 11.8478 10.8065 11.7893C9.90958 11.2466 8.79839 10.9615 7.59422 10.9615C6.97974 10.9615 6.31656 11.0341 5.62328 11.1757L5.53917 11.197C5.45062 11.2182 5.3612 11.2412 5.29213 11.2412C5.22675 11.2422 5.16184 11.2301 5.10121 11.2056C5.04059 11.1811 4.98547 11.1447 4.93911 11.0986C4.89275 11.0524 4.85607 10.9975 4.83125 10.937C4.80642 10.8765 4.79393 10.8117 4.79453 10.7463C4.79453 10.4647 4.95391 10.2655 5.21865 10.2151C5.99856 10.0322 6.7967 9.9383 7.59776 9.93526C8.99672 9.93526 10.2487 10.2584 11.3174 10.9004C11.5016 11.0075 11.6167 11.1306 11.6167 11.4042C11.6167 11.5348 11.5648 11.6601 11.4725 11.7526C11.3802 11.845 11.255 11.8971 11.1244 11.8973V11.8973ZM11.8132 9.96448C11.6565 9.96448 11.5547 9.90516 11.4635 9.85203C9.8476 8.89224 7.4375 8.57438 5.45594 9.10208C5.42577 9.11102 5.39566 9.12017 5.36562 9.12953C5.29125 9.15344 5.22042 9.17646 5.12302 9.17646C4.96584 9.17576 4.81536 9.1127 4.70463 9.00114C4.59391 8.88958 4.53198 8.73864 4.53245 8.58146C4.53245 8.26536 4.69714 8.04401 4.99641 7.9599C5.81341 7.72312 6.66039 7.60591 7.51099 7.61193C9.18089 7.61193 10.7994 8.03073 12.0673 8.78953C12.2984 8.92057 12.4029 9.10208 12.4029 9.37214C12.4029 9.69974 12.1382 9.96448 11.8132 9.96448V9.96448ZM12.5951 7.76156C12.4629 7.76196 12.3334 7.72414 12.2223 7.65266C11.1253 6.99391 9.41464 6.60255 7.64115 6.60255C6.71766 6.60255 5.87917 6.70437 5.14693 6.90182C5.1222 6.90809 5.09769 6.91517 5.07344 6.92307C4.98429 6.95333 4.89125 6.97063 4.79719 6.97443C4.70537 6.97479 4.6144 6.95684 4.5296 6.92163C4.4448 6.88642 4.36787 6.83465 4.30332 6.76935C4.23877 6.70406 4.18788 6.62655 4.15364 6.54135C4.1194 6.45615 4.10249 6.36499 4.10391 6.27318C4.10391 5.93141 4.29604 5.67021 4.61656 5.57547C5.50021 5.31427 6.51844 5.18323 7.64026 5.18323C9.6599 5.18323 11.583 5.63125 12.9147 6.41484C13.1661 6.55651 13.2892 6.77255 13.2892 7.07182C13.2898 7.16294 13.2722 7.25326 13.2375 7.33751C13.2028 7.42175 13.1516 7.49822 13.087 7.56244C13.0223 7.62667 12.9455 7.67736 12.8611 7.71155C12.7766 7.74574 12.6862 7.76274 12.5951 7.76156V7.76156Z"
                    fill=if fill.is_empty() { "#07C330" } else { "white" }
                />
            </g>
            <defs>
                <filter
                    id="filter0_d"
                    x="0"
                    y="0"
                    width="44"
                    height="44"
                    filterUnits="userSpaceOnUse"
                    color-interpolation-filters="sRGB"
                >
                    <feFlood flood-opacity="0" result="BackgroundImageFix" />
                    <feColorMatrix
                        in="SourceAlpha"
                        type="matrix"
                        values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"
                    />
                    <feOffset />
                    <feGaussianBlur stdDeviation="5" />
                    <feColorMatrix
                        type="matrix"
                        values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0.69 0"
                    />
                    <feBlend mode="normal" in2="BackgroundImageFix" result="effect1_dropShadow" />
                    <feBlend
                        mode="normal"
                        in="SourceGraphic"
                        in2="effect1_dropShadow"
                        result="shape"
                    />
                </filter>
            </defs>
        </svg>
    }
}
