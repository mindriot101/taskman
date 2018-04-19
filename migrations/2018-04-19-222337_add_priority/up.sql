alter table tasks add column priority text check(priority in ('high', 'medium', 'low'))
