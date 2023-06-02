# leavebalance

extrapolate future leave balances to plan usage of leave

## Usage

Use `leavebalance on <DATE> <CONFIG_FILE> [CURRENT_LEAVE_BALANCE] [NEXT_PAY_DAY] [BALANCE_WARN_THRESHOLD]` to calculate leave balance at some future date, using a configuration file:
```shell
leavebalance on 2023-09-04 examples/example_1.yaml
```
```
your leave balance will be 56 hours (7 working days) on 2023-09-04
```

```shell
leavebalance on 2023-12-01 examples/example_2.yaml 95.7688
```
```
your leave balance will be 87 hours (10 working days) on 2023-08-30

```

The configuration file looks like this:
```yaml
policy:
  working_hours: [08:00:00, 16:00:00]
  working_days: ['Monday', 'Tuesday', 'Wednesday', 'Thursday', 'Friday']
  days_in_pay_period: 14
  hours_accrued_per_pay_period: 8

planned_leave:
    - [2023-06-09, 2023-06-09]
    - [2023-06-27, 2023-07-04]
    - [2023-08-04, 2023-08-11]
    - [2023-11-30, 2023-12-04]
    - [2024-04-08, 2024-04-08]
    - [2024-04-22, 2024-04-26]
```

You can also quickly generate a configuration file with `leavebalance config <FILENAME> <HOURS_ACCRUED_PER_PAY_PERIOD>`:
```shell
leavebalance config ./myconfig.yaml 5.3
leavebalance on 2023-06-30 ./myconfig.yaml 10.5
```
```
your leave balance will be 26 hours (3 working days) on 2023-06-30
```