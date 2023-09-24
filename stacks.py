import argparse
import datetime

# Instantiate the parser
parser = argparse.ArgumentParser(description='Task management system based on the idea of a dependency stack.')

parser.add_argument('--list', action='store_true',
                    help='List all pending tasks.')

parser.add_argument('--list-all', action='store_true',
                    help='List all tasks (pending and done).')

parser.add_argument('--list-leaves', action='store_true',
                    help='List all tasks that do not have dependencies.')

parser.add_argument('--delete', type=int,
                    help='Delete a task by the identifier.')

parser.add_argument('--add', action='store_true',
                    help='Add a new task.')

parser.add_argument('--root', action='store_true',
                    help='Specity that the new task does not depend on any existing task.')

parser.add_argument('title', type=str, nargs='?',
                    help='Title of the newly added task.')

parser.add_argument('due', type=lambda s: datetime.datetime.strptime(s, '%d-%m-%Y'), nargs='?',
                    help='Due date of the newly added task.')

args = parser.parse_args()

if args.list:
    