<details class="pokemon-card-container">
<summary>Togedemaru (#272)</summary>
Types: Electric / Steel • Egg Groups: Field / Fairy

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Spiked Shell
- Lightning Rod
- Sturdy *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Electric (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Poison (0×)
- Flying (0.25×)
- Psychic (0.5×)
- Bug (0.5×)
- Rock (0.5×)
- Dragon (0.5×)
- Steel (0.25×)
- Fairy (0.5×)

*Weak to*
- Fire (2×)
- Fighting (2×)
- Ground (4×)

**TM/HM Moves**
- TM01 - Wish
- TM06 - Toxic
- TM17 - Protect
- TM20 - Poison Jab
- TM24 - Thunderbolt
- TM25 - Thunder
- TM32 - Double Team
- TM33 - Reflect
- TM34 - Shock Wave
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM58 - Thunder Wave

**Held Item**
Electric Seed

**Encounter Locations**
- Kaptara Island (East) — Grass (Night) (4%)
- Marmaro Island — Grass (Night) (8%)
- Pollen Road — Grass (Night) (10%)
- Wanderer's Woods (North) — Grass (Night) (4%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="togedemaru" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">98</span> |
| Defense | <span class="stat-value stat-mid">83</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">73</span> |
| Speed | <span class="stat-value stat-high">96</span> |
| Total | <span class="stat-value stat-mid">475</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Thunder Shock (Lv 1)
- Defense Curl (Lv 5)
- Rollout (Lv 9)
- Charge (Lv 13)
- Spark (Lv 17)
- Nuzzle (Lv 21)
- Magnet Rise (Lv 25)
- Discharge (Lv 29)
- Zing Zap (Lv 33)
- Iron Head (Lv 35)
- Electric Terrain (Lv 37)
- Wild Charge (Lv 41)
- Pin Missile (Lv 43)
- Spiky Shield (Lv 46)
- Fell Stinger (Lv 48)
- Thunderclap (Lv 55)

**Egg Moves**
- Reversal
- Present
- Encore
- Twineedle
- Wish
- Fake Out
- Tickle
- Flail
- Disarming Voice

**Tutor Moves**
- Defense Curl
- Endure
- Rollout
- Sleep Talk
- Snore
- Swagger
- Swift
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
</details>
