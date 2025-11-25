#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "raindrops.h"

static bool divisible_by(int n, int d)
{
   return n % d == 0;
}

typedef struct
{
   uint8_t factor;
   char *sound;
} RainDropFactorAndNoise_t;

#define NUM_ELEMENTS(array) (sizeof(array) / sizeof(array[0]))

void convert(char result[], int drops)
{
   RainDropFactorAndNoise_t factorsAndSounds[] = {
      { .factor = 3, .sound = "Pling" },
      { .factor = 5, .sound = "Plang" },
      { .factor = 7, .sound = "Plong" },

   };
   uint8_t cursor = 0;
   for(uint8_t i = 0; i < NUM_ELEMENTS(factorsAndSounds); i++)
   {
      if(divisible_by(drops, factorsAndSounds[i].factor))
      {
         strcat(&result[cursor], factorsAndSounds[i].sound);
         cursor += strlen(factorsAndSounds[i].sound);
      }
   }

   if(cursor == 0)
   {
      sprintf(result, "%d", drops);
   }
}
