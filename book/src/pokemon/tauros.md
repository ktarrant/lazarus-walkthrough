<details class="pokemon-card-container">
<summary>Tauros (#244)</summary>
Types: Normal • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Anger Point
- Sheer Force *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Ghost (0×)

*Weak to*
- Fighting (2×)

**TM/HM Moves**
- TM03 - Water Pulse
- TM06 - Toxic
- TM07 - Whirlpool
- TM11 - Sunny Day
- TM13 - Ice Beam
- TM14 - Blizzard
- TM17 - Protect
- TM18 - Rain Dance
- TM22 - Solar Beam
- TM24 - Thunderbolt
- TM25 - Thunder
- TM26 - Earthquake
- TM28 - Dig
- TM30 - Shadow Ball
- TM32 - Double Team
- TM34 - Shock Wave
- TM35 - Flamethrower
- TM37 - Sandstorm
- TM38 - Fire Blast
- TM39 - Rock Tomb
- TM42 - Facade
- TM44 - Rest
- TM45 - Attract
- TM46 - Thief
- TM49 - Bulldoze
- HM03 - Surf
- HM04 - Strength
- HM06 - Rock Smash

**Encounter Locations**
- Asfal Hills — Grass (Day) (20%)
- Kaptara Island (East) — Grass (Day) (10%)
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="tauros" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">75</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-high">95</span> |
| Sp. Atk | <span class="stat-value stat-low">40</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">110</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- Tackle (Lv 1)
- Tail Whip (Lv 3)
- Rage (Lv 5)
- Horn Attack (Lv 8)
- Scary Face (Lv 11)
- Pursuit (Lv 15)
- Rest (Lv 19)
- Payback (Lv 24)
- Work Up (Lv 29)
- Horn Leech (Lv 33)
- Body Slam (Lv 35)
- Zen Headbutt (Lv 41)
- Stomping Tantrum (Lv 43)
- Swagger (Lv 48)
- Thrash (Lv 55)
- Double-Edge (Lv 63)
- Giga Impact (Lv 71)

**Egg Moves**
- Incompatible

**Tutor Moves**
- Body Slam
- Double-Edge
- Endure
- Icy Wind
- Rock Slide
- Sleep Talk
- Snore
- Swagger
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
