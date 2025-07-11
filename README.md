# Finance App

## Database

### A Note On Dates

To simplify things, and remove ambiguity, dates are handled in a bespoke way.

All dates are converted into an eight digit number. This is effectively done by removing the dashes in an ISO date.
Here's an example.

`2025-05-03` would become `20250503`.

The reason is very simple. Times are meaningless for this system, only the date matters. Using a system that generally
deals in dates and times becomes way more complex than it needs to be.

I may try a simple method of using "days since epoch" as well but that's more math than I want to do right now.

According to google, four countries don't use a calendar that works with ISO.

### Integer Types

In many places where I *could* use more advanced integer types (such as `u16`) I am not. This is because postgres
doesn't have unsigned integers.

Anywhere that money is involved, I'm using an `i64`. Decimals don't exist in the data. They can be represented in the UI
by simply dividing the value by 100. This is done to simplify translation of the data.

## Open Source

While I am using a FOSS license (one specifically noted by the OSI), this software is more or less just public. I am not
looking for PRs, and will not accept them. If you would like a feature, for it and implement it yourself.
