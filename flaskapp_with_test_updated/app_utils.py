from datetime import datetime


class WorkShiftFormatter:
    def __init__(self, opening_hours):
        """
        Formats opening hours to be human readable

        Example Input:
        {
            "thursday" : [], # Empty list means the store is closed for the day
            "friday" : [
                {
                    "type" : "open",
                    "value" : 64800,  # open: Friday: 6 PM - 1 AM Saturday
                }
            ],
            "saturday": [
                {
                    "type" : "close",
                    "value" : 3600
                },
                {
                    "type" : "open",
                    "value" : 32400
                },
                {
                    "type" : "close",
                    "value" : 39600
                },
            ]
        }

        Example Output:
        {
            "friday": "06:00 PM - 01:00 AM",
            "saturday": "09:00 AM - 11:00 AM",
            "thursday": "Closed"
        }
        """
        self.opening_hours = opening_hours

    def to_human_readable(self):
        day_formatted = {}

        for day_of_week, opening_hours in self.opening_hours.items():
            if len(opening_hours) == 0:
                day_formatted[day_of_week] = 'Closed'
                continue

            opening_hours_sorted = sorted(
                opening_hours,
                key=lambda x: x['value'],
            )

            first_hour_is_closed = opening_hours_sorted[0]['type'] == 'close'
            if first_hour_is_closed:
                opening_hours_sorted.pop(0)

            last_hour_is_open = opening_hours_sorted[-1]['type'] == 'open'
            if last_hour_is_open:
                first_hour_next_day = min(
                    self.opening_hours[self.__get_next_day(day_of_week)],
                    key=lambda x: x['value'],
                )

                # There shouldn't be contiguous opening hours
                if first_hour_next_day['type'] == 'open':
                    raise ValueError(f'opening time on {day_of_week} does not have a closing time',)

                opening_hours_sorted.append(first_hour_next_day)

            day_formatted[day_of_week] = ', '.join(
                self.__to_human_readable_opening_hours(opening_hours_sorted))
        return day_formatted

    def __get_next_day(self, day):
        days = [
            'monday',
            'tuesday',
            'wednesday',
            'thursday',
            'friday',
            'saturday',
            'sunday',
            'monday',
        ]
        assert day in days

        next_day_index = days.index(day) + 1
        return days[next_day_index]

    def __to_human_readable_time(self, timestamp):
        return datetime.utcfromtimestamp(timestamp).strftime('%I:%M %p')

    def __to_human_readable_opening_hours(self, opening_hours_sorted):
        time_list = []
        for i in range(0, len(opening_hours_sorted), 2):
            start = opening_hours_sorted[i]['value']
            end = opening_hours_sorted[i + 1]['value']
            time_list.append(f'{self.__to_human_readable_time(start)} - {self.__to_human_readable_time(end)}')

        return time_list

