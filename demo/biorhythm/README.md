# Biorhythm Calculator

This program is based on the Biorhythm program from the book "TRS-80 Programs" by Tom Rugg and Phil Feldman. It has been modernized and adapted to run on current systems.

## Introduction

Have you ever had one of those days when nothing seemed to go right? When you felt clumsy, depressed, or just couldn't concentrate? While sometimes we know the reason (like coming down with a cold or having an argument), other times it seems to happen for no apparent reason.

Biorhythm theory suggests that all of us have cycles, beginning at birth, that influence our physical, emotional, and intellectual states. This program allows you to explore these cycles for any given time period.

According to biorhythm theory:

1. The physical cycle is 23 days long. The first 11.5 days are the "positive" half, associated with higher energy and endurance. The second 11.5 days are the "negative" half, often accompanied by fatigue and lower physical performance.

2. The emotional cycle lasts 28 days. The positive half (first 14 days) tends to bring cheerfulness and optimism, while the negative half may lead to moodiness and irritability.

3. The intellectual cycle spans 33 days. During the positive half, you might find it easier to learn new things and engage in creative activities. The negative half might be better suited for reviewing familiar material.

The program calculates and displays these cycles for a specified period, allowing you to see how they interact and potentially influence your daily life.

## How to Use

1. When prompted, enter your birth date (YYYY-MM-DD).
2. Enter the start date for the chart (YYYY-MM-DD).
3. Optionally, specify the number of days to chart (default is 365).

The program will then display a chart showing the state of each cycle (physical, emotional, and intellectual) for each day in the specified period. 

- '+' indicates a positive phase
- '-' indicates a negative phase
- '0' indicates a critical day (transition between positive and negative)

The chart is color-coded for easy interpretation:
- Green: All cycles positive
- Yellow: Two cycles positive
- Pink: One cycle positive
- Red: All cycles negative
- Green background: Triple positive day (all cycles highly positive)
- Red background: Triple negative day (all cycles highly negative)

Please note that while biorhythm theory is interesting, it's not scientifically proven. Many scientists argue that there isn't enough evidence to support its claims. Others believe biorhythms exist but may not follow such rigid cycles. This program is provided as an interesting way to explore the concept, not as a scientifically validated tool.

Enjoy exploring your biorhythms!

## License

This code is open source and released under the MIT License. You are free to use, modify, and distribute this software, subject to the terms and conditions of the MIT License.

Source: The Rust Bucket (https://github.com/bytecoder-git/rust-bucket)
