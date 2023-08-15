# leavebalance

extrapolate future leave balances to plan usage of leave

## Usage

Use `leavebalance on <DATE> <CONFIG_FILE> [CURRENT_LEAVE_BALANCE] [NEXT_PAY_DAY] [BALANCE_WARN_THRESHOLD]` to calculate leave balance at some future date, using a configuration file:
```shell
leavebalance on 2023-09-04 examples/example_1.toml
```
```
56.00
```
```shell
leavebalance on 2023-12-01 examples/example_2.toml 95.7688
```
```
135.77
```
If your starting balance is too low, a warning is printed to stderr:

```shell
leavebalance on 2023-08-30 examples/example_2.toml 20
```
```
your planned leave on 2023-08-10 would deplete your leave balance to -4 hours!
12.00
```
You can also use `--verbose` to diagnose individual usages and accruals by date:
```shell
leavebalance on 2023-08-30 examples/example_2.toml 20 --verbose
```
```
2023-06-02 - 8.00 hours accrued; balance is now 20.00 hours
2023-06-16 - 8.00 hours accrued; balance is now 28.00 hours
2023-06-28 - 8.00 hours used;    balance is now 28.00 hours
2023-06-29 - 8.00 hours used;    balance is now 20.00 hours
2023-06-30 - 8.00 hours accrued; balance is now 20.00 hours
2023-06-30 - 8.00 hours used;    balance is now 20.00 hours
2023-07-03 - 8.00 hours used;    balance is now 12.00 hours
2023-07-14 - 8.00 hours accrued; balance is now 12.00 hours
2023-07-28 - 8.00 hours accrued; balance is now 20.00 hours
2023-08-07 - 8.00 hours used;    balance is now 20.00 hours
2023-08-08 - 8.00 hours used;    balance is now 12.00 hours
2023-08-09 - 8.00 hours used;    balance is now 4.00 hours
2023-08-10 - 8.00 hours used;    balance is now -4.00 hours
your planned leave on 2023-08-10 would deplete your leave balance to -4 hours!
2023-08-11 - 8.00 hours accrued; balance is now -4.00 hours
2023-08-25 - 8.00 hours accrued; balance is now 4.00 hours
accrued 56.00 hours and used 64.00 hours
12.00
```

The configuration file looks like this:
```toml
[plans]
paid_leave = [
   ["2023-06-09", "2023-06-09"],
   ["2023-06-27", "2023-07-04"],
   ["2023-08-04", "2023-08-11"],
   ["2023-11-30", "2023-12-04"],
   ["2024-04-08", "2024-04-08"],
   ["2024-04-22", "2024-04-26"],
]

[policy]
days_in_pay_period = 14
hours_accrued_per_pay_period = 8

[policy.work_week]
Monday = ["08:00:00", "16:00:00"]
Tuesday = ["08:00:00", "16:00:00"]
Wednesday = ["08:00:00", "16:00:00"]
Thursday = ["08:00:00", "16:00:00"]
Friday = ["08:00:00", "16:00:00"]
```

You can also quickly generate a default configuration file with `leavebalance config <FILENAME> <HOURS_ACCRUED_PER_PAY_PERIOD>`:
```shell
leavebalance config ./my_leave_policy.toml
```
```toml
[plans]
paid_leave = []

[policy]
days_in_pay_period = 14
hours_accrued_per_pay_period = 0.0

[policy.work_week]
Mon = [
    "09:00:00",
    "17:00:00",
]
Tue = [
    "09:00:00",
    "17:00:00",
]
Wed = [
    "09:00:00",
    "17:00:00",
]
Fri = [
    "09:00:00",
    "17:00:00",
]
Thu = [
    "09:00:00",
    "17:00:00",
]
```
```shell
leavebalance on 2023-06-30 ./my_leave_policy.toml 10.5
```
```
10.50
```