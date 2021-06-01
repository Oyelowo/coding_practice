from app_utils import WorkShiftFormatter

from flask import Flask, request

app = Flask(__name__)
app.config['DEBUG'] = True


@app.route('/', methods=['GET'])
def home():
    # ideally, this input json should be validated, typically
    # by using standard JSON schema validation.
    # Contiguity check should also be done to ensure that
    # there are no contiguous opening or closing hours in the
    # list. e.g [{type: "open", value: <...>}, {type: "open", value:  <...>]
    opening_hours = request.get_json('opening_hours')
    opening_hours = WorkShiftFormatter(opening_hours).to_human_readable()

    return f"""
            <ul>
                <li>Monday: {opening_hours['monday']}</li>
                <li>Tuesday: {opening_hours['tuesday']}</li>
                <li>Wednesday: {opening_hours['wednesday']}</li>
                <li>Tuesday: {opening_hours['thursday']}</li>
                <li>Friday: {opening_hours['friday']}</li>
                <li>Saturday: {opening_hours['saturday']}</li>
                <li>Sunday: {opening_hours['sunday']}</li>
            </ul>

            """

app.run()
