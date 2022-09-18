# Plan for Stocks App

Author: Adrian Tarantino

Created: September 17, 2022
Last Updated: September 18, 2022

### Intentions of The Project

- Practice using Rust's Rocket framework
- Practice making GUIs in html, css, js

### Core functionality

- Grab stock data from Binance's free API
- Display the data on an HTML page

### Similar Projects

> 1. [crypto_discord_bot](https://github.com/AdrianTheHacker/crypto_discord_bot)
    - Made by me (AdrianTheHacker)

### Tools
##### Backend
> 1. Rust programming language

> 2. Rocket Web Framework
    - This is a web framework for Rust

##### Frontend
> 1. HTML, CSS, JS

##### External APIs
> 1. [Binances free api](https://www.binance.com/en/markets)

##### Other Dependencies
None at the moment


### Implementation Plan
##### Step 1
1. Working Backend 😎

2. Grab stock data from binance's API 😎
    - May have to use tokio::task::spawn_blocking 

3. Display the stock data that has been searched 😎

##### Step 2
2. Displaying frontend 😎

##### Step 3 
1. Add search bar

2. Allow users to ping certain currencies to a table

##### Step 4
1. Deploy the site (good luck)


### Changes to Plan
##### September 18, 2022
> 1. Switching from Rocket to Yew
    - This project now uses web assembly
    - Rocket over complicated the frontend

> 2. HTTP requests are now made using gloo_net instead of reqwests
    - Yew can't use reqwest::blocking
    - Yew's website has a great [tutorial on how to make HTTP requests using gloo_net](https://yew.rs/docs/tutorial#fetching-data-using-external-rest-api)


