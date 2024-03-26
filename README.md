# leavebalance

extrapolate future leave balances to plan usage of leave

### Instructions

1. Write your leave policy and planned leave in a configuration file in TOML format (`.toml`):
```toml
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

[plans.paid]
leave = [
   ["2023-06-09", "2023-06-09"],
   ["2023-06-27", "2023-07-04"],
   ["2023-08-04", "2023-08-11"],
   ["2023-11-30", "2023-12-04"],
   ["2024-04-08", "2024-04-08"],
   ["2024-04-22", "2024-04-26"],
]
```

2. Download and unzip an executable from the [latest release on the Releases page](https://github.com/zacharyburnett/leavebalance/releases).

3. Open a Terminal window

4. Run the executable with the `on` command (`leavebalance on [OPTIONS] <DATE> <CONFIG_FILE>`) to calculate your leave balance on some future date. Include the `--from` and `--next-pay-day` arguments to define start dates (both default to the current date):
```shell
leavebalance on 2023-09-04 examples/example_1.toml --from 2023-06-04 --next-pay-day 2023-06-09
```
Include the `--balance` argument to input a current (or starting) leave balance:
```
56.00
```
```shell
leavebalance on 2023-12-01 examples/example_2.toml --balance 95.7688 --from 2023-06-04 --next-pay-day 2023-06-09
```
```
127.8
```

> [!NOTE]
> If your starting balance is too low, a warning is printed to stderr:
> ```shell
> leavebalance.exe on 2023-08-30 examples/example_2.toml --balance 20 --from 2023-06-04 --next-pay-day 2023-06-09
> ```
> ```
> your planned leave on 2023-08-10 would deplete your leave balance to -4.0h!
> 4.0
> ```

> [!TIP]
> You can also use `--verbose` to show daily usages / accruals of leave:
> ```shell
> leavebalance.exe on 2023-08-30 examples/example_2.toml --balance 20 --from 2023-06-01 --next-pay-day 2023-06-02 --verbose
> ```
> 
> ```
> 2023-06-02 +8.0h => 28.0h
> 2023-06-16 +8.0h => 36.0h
> 2023-06-28 -8.0h => 28.0h
> 2023-06-29 -8.0h => 20.0h
> 2023-06-30 +8.0h => 28.0h
> 2023-06-30 -8.0h => 20.0h
> 2023-07-03 -8.0h => 12.0h
> 2023-07-14 +8.0h => 20.0h
> 2023-07-28 +8.0h => 28.0h
> 2023-08-07 -8.0h => 20.0h
> 2023-08-08 -8.0h => 12.0h
> 2023-08-09 -8.0h => 4.0h
> 2023-08-10 -8.0h => -4.0h
> your planned leave on 2023-08-10 would deplete your leave balance to -4.0h!
> 2023-08-11 +8.0h => 4.0h
> 2023-08-25 +8.0h => 12.0h
> accrued 56.0h and used 64.0h
> 12.0
> ```

> [!TIP]
> You can quickly generate a default configuration file with `leavebalance write <FILENAME>`:
> ```shell
> leavebalance.exe write my_leave_policy.toml
> ```
> ```toml
> [policy]
> days_in_pay_period = 14
> hours_accrued_per_pay_period = 0.0
> country = "US"
> paid_holidays = []
> 
> [policy.work_week]
> Tue = ["09:00:00", "17:00:00"]
> Fri = ["09:00:00", "17:00:00"]
> Wed = ["09:00:00", "17:00:00"]
> Mon = ["09:00:00", "17:00:00"]
> Thu = ["09:00:00", "17:00:00"]
> 
> [plans.paid]
> leave = []
> ```
> ```shell
> leavebalance.exe on 2023-06-30 my_leave_policy.toml --balance 10.5 --from 2023-06-01
> ```
> ```
> 10.50
> ```
