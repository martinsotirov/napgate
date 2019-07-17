# NAPGate

On 16.07.2019 hundreds of databases of the Bulgarian tax agency [got leacked](https://www.reuters.com/article/us-bulgaria-cybersecurity/hackers-hit-bulgaria-leak-data-from-russian-email-government-idUSKCN1UB0MA) by a Russian hacker. This tool allows the extraction of some data from the .csv files. Purely for analytics purposes.

## Install
```
$ cargo build --release
```

## Usage

To extract unique email addresses from the leaked CSV files run
```
$ napgate extract emails -o output_file.txt /path/to/csv/folder
```

To extract EGN (unique civil number) run
```
$ napgate extract egn -o output_file.txt /path/to/csv/folder
```

Alternatively the `-o` flag can be omitted to just get a number of unique entries.

## Analytics
Running the tool on the current amount of leaked files results in:

* 484654 email addresses
* 5216168 EGN
