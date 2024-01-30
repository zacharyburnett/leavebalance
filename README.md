# leavebalance

extrapolate future leave balances to plan usage of leave

## Usage

Use `leavebalance on <DATE> <CONFIG_FILE> [CURRENT_LEAVE_BALANCE] [NEXT_PAY_DAY] [BALANCE_WARN_THRESHOLD]` to calculate leave balance at some future date, using a configuration file:
```shell
leavebalance on 2023-09-04 examples/example_1.toml --from 2023-06-04 --next-pay-day 2023-06-09
```
```
56.00
```
```shell
leavebalance on 2023-12-01 examples/example_2.toml --balance 95.7688 --from 2023-06-04 --next-pay-day 2023-06-09
```
```
127.8
```
If your starting balance is too low, a warning is printed to stderr:

```shell
leavebalance on 2023-08-30 examples/example_2.toml --balance 20 --from 2023-06-04 --next-pay-day 2023-06-09
```
```
your planned leave on 2023-08-10 would deplete your leave balance to -4.0h!
4.0
```
You can also use `--verbose` to diagnose individual usages and accruals by date:
```shell
leavebalance on 2023-08-30 examples/example_2.toml --balance 20 --from 2023-06-01 --next-pay-day 2023-06-02 --verbose
```
```
2023-06-02 +8.0h => 28.0h
2023-06-16 +8.0h => 36.0h
2023-06-28 -8.0h => 28.0h
2023-06-29 -8.0h => 20.0h
2023-06-30 +8.0h => 28.0h
2023-06-30 -8.0h => 20.0h
2023-07-03 -8.0h => 12.0h
2023-07-14 +8.0h => 20.0h
2023-07-28 +8.0h => 28.0h
2023-08-07 -8.0h => 20.0h
2023-08-08 -8.0h => 12.0h
2023-08-09 -8.0h => 4.0h
2023-08-10 -8.0h => -4.0h
your planned leave on 2023-08-10 would deplete your leave balance to -4.0h!
2023-08-11 +8.0h => 4.0h
2023-08-25 +8.0h => 12.0h
accrued 56.0h and used 64.0h
12.0
```

The configuration file looks like this:
```toml
[plans.paid]
leave = [
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
country = "US"
paid_holidays = [
   "New Year's Day",
   "Martin Luther King Jr. Day",
   "Memorial Day",
   "Juneteenth",
   "Independence Day",
   "Labor Day",
   "Thanksgiving",
   "Christmas Day",
]

[policy.work_week]
Monday = ["08:00:00", "16:00:00"]
Tuesday = ["08:00:00", "16:00:00"]
Wednesday = ["08:00:00", "16:00:00"]
Thursday = ["08:00:00", "16:00:00"]
Friday = ["08:00:00", "16:00:00"]
```

You can also quickly generate a default configuration file with `leavebalance config <FILENAME> <HOURS_ACCRUED_PER_PAY_PERIOD>`:
```shell
leavebalance write ./my_leave_policy.toml
```
```toml
[policy]
days_in_pay_period = 14
hours_accrued_per_pay_period = 0.0
country = "US"
paid_holidays = []

[policy.work_week]
Tue = [
    "09:00:00",
    "17:00:00",
]
Fri = [
    "09:00:00",
    "17:00:00",
]
Wed = [
    "09:00:00",
    "17:00:00",
]
Mon = [
    "09:00:00",
    "17:00:00",
]
Thu = [
    "09:00:00",
    "17:00:00",
]

[plans.paid]
leave = []
```
```shell
leavebalance on 2023-06-30 ./my_leave_policy.toml --balance 10.5 --from 2023-06-01
```
```
10.50
```
