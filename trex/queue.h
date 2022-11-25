#include "HardwareSerial.h"
typedef void (*TaskCallback)();

struct Task
{
  unsigned long start;
  unsigned long delay;
  TaskCallback func;
};

const size_t TASK_POOL_SIZE = 2;

static const struct Task *TaskPool[TASK_POOL_SIZE];

void clear_queue()
{
  // insert task to the task pool
  for (size_t i = 0; i < TASK_POOL_SIZE; i++)
  {

    free(&TaskPool[i]);
    TaskPool[i] = NULL;
  }
}

void waitcall(TaskCallback func, unsigned long delay)
{

  Serial.print("HERE");
  
  Task *task = (Task *) malloc(sizeof(Task));
  task->delay = delay;
  task->func = func;
  task->start = millis();

  // insert task to the task pool
  for (size_t i = 0; i < TASK_POOL_SIZE; i++)
    if (TaskPool[i] == NULL || TaskPool[i] == nullptr)
    {
      TaskPool[i] = task;
      return;
    }

  Serial.println(F("NO MORE SPACE IN TASK POOL"));
}

// put this in the loop function
// checks to see what tasks should be done, runs them and cleans them up
void doChores()
{

  for (size_t i = 0; i < TASK_POOL_SIZE; i++)
  {
    if (TaskPool[i] == NULL || TaskPool[i] == nullptr)
      continue;

    if ((TaskPool[i]->start + TaskPool[i]->delay) < millis())
    {
      // run task

      // Serial.print(F("Start: "));
      // Serial.println(TaskPool[i]->start);
      // Serial.print(F("Delay: "));
      // Serial.println(TaskPool[i]->delay);
      // Serial.print(F("Now: "));
      // Serial.println(millis());
      // Serial.print(F("Index: "));
      // Serial.println(i);

      // Serial.println(F("------------RAN-------------"));

      TaskPool[i]->func();

      free(&TaskPool[i]);
      TaskPool[i] = NULL;
    }
  }
}