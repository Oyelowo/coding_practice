# PART ONE
## STEPS TO RUN THE APPLICATION 

1. Make sure you have docker installed. Find instructions at https://docs.docker.com/docker-for-mac/install/

2. RUN `docker-compose up`


## Testing the application.
After starting the application, you can run the below command with curl.

```
curl -X GET http://localhost:5000/ -H "Content-Type: application/json" -d '{
  "monday": [],
  "tuesday": [
    {
      "type": "open",
      "value": 36000
    },
    {
      "type": "close",
      "value": 64800
    }
  ],
  "wednesday": [],
  "thursday": [
    {
      "type": "open",
      "value": 37800
    },
    {
      "type": "close",
      "value": 64800
    }
  ],
  "friday": [
    {
      "type": "open",
      "value": 36000
    }
  ],
  "saturday": [
    {
      "type": "close",
      "value": 3600
    },
    {
      "type": "open",
      "value": 36000
    }
  ],
  "sunday": [
    {
      "type": "close",
      "value": 3600
    },
    {
      "type": "open",
      "value": 43200
    },
    {
      "type": "close",
      "value": 75600
    }
  ]
}
'

```


## RUNNING test
RUN `python test.py`

# PART TWO
Tell us what you think about the data format. Is the current JSON structure the best way
to store that kind of data or can you come up with a better version? There are no right
answers here 🙂. Please write your thoughts to readme.md.

## ANSWER
One advantage of the input schema approach provided in the task is that one can validate that there is no
contiguous opening or closing hours relatively easily by sorting by the timestamp
values and ensuring that opening/closing hours are not next to each other.
e.g {"type" : "open","value" : 3230}, {"type" : "open", "value" : 32400 }.
After sorting and we get something like the above example. This way, we can tell that
the data is invalid because "type:open" is next to "type:open".

However, this approach could be a bit cumbersome to work with. A more intuitive 
API, in my opinion would be to colocate the opening and closing hours in each object of the list.
This way, validation can be done more easily by simply validating that the times don't overlap in each list.
Also, less data mapping would be required as one can simply map through each list
to get all necessary information to format the data rather than e.g visiting the next day
to get the closing time if opening time starts the previous day. In other words, the input data would be
closer to the destination output format.

Furthermore, I would consider using UTC time(ISO 8601 format) for the time e.g 10:30Z as shown below..
This way, we can account for timezones.

Fig1: co-locating open and close time in each object and using UTC time
```
{
    monday: [
        {
            open: 09:30Z,
            close: 16:00Z
        },
        {
            open: 23:30Z,
            # # Even though this falls to the next day, for our use-case, I think it's fine to have it co-located with the same day
            # We can say that the shift belongs to the day it started. However, this won't work if
            # a shift can be longer than 24 hours.
            close: 05:00Z ,
        },
    ]
}
```


However, if a shift can be longer than 24 hours. I would consider having just the opening times and the duration.
This way we can easily invalidate overlaps by using the below simple heuristics that:
```
   present_opening_time + duration < next_opening_time
```
e.g in the below on Monday: 09:30 + 40 hours should be less than 16:30 on Wednesday
{
    monday: [
        {
            open: 09:30Z,
            duration_in_minutes: 2400, # equivalent 40 hours
        },
    ],
    wednesday: [
        {
            open: 16:30Z,
            duration_in_minutes: 300,
        },
    ]
}
