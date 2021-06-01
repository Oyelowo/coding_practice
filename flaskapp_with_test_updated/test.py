import unittest

from app_utils import WorkShiftFormatter


class TestStringMethods(unittest.TestCase):

    def test_opening_hours(self):
        opening_hours_raw_test_data = {
            'monday': [],
            'tuesday': [
                {
                    'type': 'open',
                    'value': 36000,
                },
                {
                    'type': 'close',
                    'value': 64800,
                }
            ],
            'wednesday': [],
            'thursday': [
                {
                    'type': 'open',
                    'value': 37800,
                },
                {
                    'type': 'close',
                    'value': 64800,
                }
            ],
            'friday': [
                {
                    'type': 'open',
                    'value': 36000,
                },
            ],
            'saturday': [
                {
                    'type': 'close',
                    'value': 3600,
                },
                {
                    'type': 'open',
                    'value': 36000,
                }
            ],
            'sunday': [
                {
                    'type': 'close',
                    'value': 3600,
                },
                {
                    'type': 'open',
                    'value': 43200,
                },
                {
                    'type': 'close',
                    'value': 75600,
                }
            ]
        }

        formatted_input = WorkShiftFormatter(opening_hours_raw_test_data).to_human_readable()

        assert formatted_input == {
            'monday': 'Closed',
            'tuesday': '10:00 AM - 06:00 PM',
            'wednesday': 'Closed',
            'thursday': '10:30 AM - 06:00 PM',
            'friday': '10:00 AM - 01:00 AM',
            'saturday': '10:00 AM - 01:00 AM',
            'sunday': '12:00 PM - 09:00 PM',
        }


if __name__ == '__main__':
    unittest.main()
