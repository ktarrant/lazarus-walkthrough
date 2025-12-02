<details class="pokemon-card-container">
<summary>Flittle (#145)</summary>
<div class="pokemon-tabs" id="pokemon-tabs-flittle">
<input type="radio" name="pokemon-tabs-flittle-group" id="pokemon-tabs-flittle-tab-0" checked>
<label for="pokemon-tabs-flittle-tab-0">Flittle</label>
<input type="radio" name="pokemon-tabs-flittle-group" id="pokemon-tabs-flittle-tab-1">
<label for="pokemon-tabs-flittle-tab-1">Espathra</label>
<div class="pokemon-tab-panels">
<div class="pokemon-tab-panel" id="pokemon-tabs-flittle-panel-0">
Types: Psychic • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Anticipation
- Frisk
- Speed Boost *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm05-psyshock">TM05 - Psyshock</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm48-skill-swap">TM48 - Skill Swap</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>

**Encounter Locations**
- Acrisia Mountains (Peak) — Grass (Day) (20%)
- Acrisia Mountains (Peak) — Grass (Night) (20%)
- Pythios Town — Grass (Day) (10%)
- Pythios Town — Grass (Night) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="flittle" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">40</span> |
| Attack | <span class="stat-value stat-low">35</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-mid">55</span> |
| Sp. Def | <span class="stat-value stat-low">44</span> |
| Speed | <span class="stat-value stat-mid">71</span> |
| Total | <span class="stat-value stat-low">285</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=peck">Peck</a> (Lv 1)
- <a href="move-lookup.html?q=confusion">Confusion</a> (Lv 5)
- <a href="move-lookup.html?q=baby-doll-eyes">Baby-Doll Eyes</a> (Lv 8)
- <a href="move-lookup.html?q=disarming-voice">Disarming Voice</a> (Lv 11)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 14)
- <a href="move-lookup.html?q=teeter-dance">Teeter Dance</a> (Lv 17)
- <a href="move-lookup.html?q=psybeam">Psybeam</a> (Lv 19)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 24)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 29)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 34)

**Egg Moves**
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a>
- <a href="move-lookup.html?q=hypnosis">Hypnosis</a>
- <a href="move-lookup.html?q=roost">Roost</a>

**Tutor Moves**
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
<div class="pokemon-tab-panel" id="pokemon-tabs-flittle-panel-1">
Types: Psychic • Egg Groups: Flying

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Opportunist
- Frisk
- Speed Boost *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fighting (0.5×)
- Psychic (0.5×)

*Weak to*
- Bug (2×)
- Ghost (2×)
- Dark (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm04-calm-mind">TM04 - Calm Mind</a>
- <a href="move-lookup.html?q=tm05-psyshock">TM05 - Psyshock</a>
- <a href="move-lookup.html?q=tm11-sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=tm16-light-screen">TM16 - Light Screen</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm18-rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=tm23-hex">TM23 - Hex</a>
- <a href="move-lookup.html?q=tm29-psychic">TM29 - Psychic</a>
- <a href="move-lookup.html?q=tm30-shadow-ball">TM30 - Shadow Ball</a>
- <a href="move-lookup.html?q=tm33-reflect">TM33 - Reflect</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm40-aerial-ace">TM40 - Aerial Ace</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm46-thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=tm48-skill-swap">TM48 - Skill Swap</a>
- <a href="move-lookup.html?q=tm54-dazzling-gleam">TM54 - Dazzling Gleam</a>
- <a href="move-lookup.html?q=tm57-roost">TM57 - Roost</a>

**Evolution Info**
Lv. 30

**Encounter Locations**
- Kaptara Island (West) — Grass (Night) (10%)
- Pollen Road — Grass (Night) (5%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="espathra" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-high">95</span> |
| Attack | <span class="stat-value stat-mid">60</span> |
| Defense | <span class="stat-value stat-mid">70</span> |
| Sp. Atk | <span class="stat-value stat-high">101</span> |
| Sp. Def | <span class="stat-value stat-mid">74</span> |
| Speed | <span class="stat-value stat-high">101</span> |
| Total | <span class="stat-value stat-mid">501</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=lumina-crash">Lumina Crash</a> (Lv Evo)
- <a href="move-lookup.html?q=growl">Growl</a> (Lv 1)
- <a href="move-lookup.html?q=peck">Peck</a> (Lv 1)
- <a href="move-lookup.html?q=drill-peck">Drill Peck</a> (Lv 1)
- <a href="move-lookup.html?q=feather-dance">Feather Dance</a> (Lv 1)
- <a href="move-lookup.html?q=confusion">Confusion</a> (Lv 5)
- <a href="move-lookup.html?q=baby-doll-eyes">Baby-Doll Eyes</a> (Lv 8)
- <a href="move-lookup.html?q=disarming-voice">Disarming Voice</a> (Lv 11)
- <a href="move-lookup.html?q=quick-attack">Quick Attack</a> (Lv 14)
- <a href="move-lookup.html?q=teeter-dance">Teeter Dance</a> (Lv 17)
- <a href="move-lookup.html?q=psybeam">Psybeam</a> (Lv 19)
- <a href="move-lookup.html?q=pluck">Pluck</a> (Lv 24)
- <a href="move-lookup.html?q=agility">Agility</a> (Lv 29)
- <a href="move-lookup.html?q=uproar">Uproar</a> (Lv 34)
- <a href="move-lookup.html?q=dazzling-gleam">Dazzling Gleam</a> (Lv 37)
- <a href="move-lookup.html?q=psychic">Psychic</a> (Lv 43)
- <a href="move-lookup.html?q=no-retreat">No Retreat</a> (Lv 45)
- <a href="move-lookup.html?q=last-resort">Last Resort</a> (Lv 50)

**Egg Moves**
- <a href="move-lookup.html?q=ally-switch">Ally Switch</a>
- <a href="move-lookup.html?q=hypnosis">Hypnosis</a>
- <a href="move-lookup.html?q=roost">Roost</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=psych-up">Psych Up</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swift">Swift</a>
</div>
</div>
<script>
(function() {
  if (window.__lazarusCaughtInit) return; window.__lazarusCaughtInit = true;
  const STORAGE_KEY = 'lazarusCaught';
  function loadState() {
    try { return JSON.parse(localStorage.getItem(STORAGE_KEY) || '{}'); } catch (_) { return {}; }
  }
  function saveState(state) {
    try { localStorage.setItem(STORAGE_KEY, JSON.stringify(state)); } catch (_) {}
  }
  function applyState() {
    const state = loadState();
    const boxes = Array.from(document.querySelectorAll('.caught-check'));
    const bySpecies = boxes.reduce((m, cb) => {
      const s = cb.dataset.species; if (!s) return m; (m[s] ||= []).push(cb); return m; }, {});
    boxes.forEach(cb => {
      const key = cb.dataset.species;
      cb.checked = !!state[key];
      cb.onchange = () => {
        const checked = cb.checked;
        if (checked) state[key] = true; else delete state[key];
        saveState(state);
        (bySpecies[key] || []).forEach(other => { if (other !== cb) other.checked = checked; });
      };
    });
  }
  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', applyState);
  } else {
    applyState();
  }
})();
</script>
</div>
</div>
</div>
<style>
#pokemon-tabs-flittle-tab-0:checked ~ .pokemon-tab-panels #pokemon-tabs-flittle-panel-0 { display: block; }
#pokemon-tabs-flittle-tab-1:checked ~ .pokemon-tab-panels #pokemon-tabs-flittle-panel-1 { display: block; }
</style>
</details>
