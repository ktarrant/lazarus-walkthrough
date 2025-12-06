<details class="pokemon-card-container">
<summary>Paldean Tauros-B (#224)</summary>
Types: Fighting / Fire • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Intimidate
- Anger Point
- Cud Chew *(Hidden)*

**Type Matchups**

*Resists / Immune to*
- Fire (0.5×)
- Grass (0.5×)
- Ice (0.5×)
- Bug (0.25×)
- Dark (0.5×)
- Steel (0.5×)

*Weak to*
- Water (2×)
- Ground (2×)
- Flying (2×)
- Psychic (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=bulk-up">TM08 - Bulk Up</a>
- <a href="move-lookup.html?q=sunny-day">TM11 - Sunny Day</a>
- <a href="move-lookup.html?q=protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=rain-dance">TM18 - Rain Dance</a>
- <a href="move-lookup.html?q=earthquake">TM26 - Earthquake</a>
- <a href="move-lookup.html?q=dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=flamethrower">TM35 - Flamethrower</a>
- <a href="move-lookup.html?q=sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=fire-blast">TM38 - Fire Blast</a>
- <a href="move-lookup.html?q=rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=thief">TM46 - Thief</a>
- <a href="move-lookup.html?q=bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=will-o-wisp">TM51 - Will-O-Wisp</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="paldean-tauros-b" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-mid">85</span> |
| Attack | <span class="stat-value stat-high">110</span> |
| Defense | <span class="stat-value stat-high">105</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-mid">70</span> |
| Speed | <span class="stat-value stat-high">100</span> |
| Total | <span class="stat-value stat-mid">500</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=tail-whip">Tail Whip</a> (Lv 1)
- <a href="move-lookup.html?q=work-up">Work Up</a> (Lv 5)
- <a href="move-lookup.html?q=double-kick">Double Kick</a> (Lv 10)
- <a href="move-lookup.html?q=flame-charge">Flame Charge</a> (Lv 15)
- <a href="move-lookup.html?q=headbutt">Headbutt</a> (Lv 20)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 25)
- <a href="move-lookup.html?q=zen-headbutt">Zen Headbutt</a> (Lv 30)
- <a href="move-lookup.html?q=raging-bull">Raging Bull</a> (Lv 35)
- <a href="move-lookup.html?q=rest">Rest</a> (Lv 40)
- <a href="move-lookup.html?q=swagger">Swagger</a> (Lv 45)
- <a href="move-lookup.html?q=thrash">Thrash</a> (Lv 50)
- <a href="move-lookup.html?q=flare-blitz">Flare Blitz</a> (Lv 55)
- <a href="move-lookup.html?q=close-combat">Close Combat</a> (Lv 60)

**Egg Moves**
- <a href="move-lookup.html?q=curse">Curse</a>
- <a href="move-lookup.html?q=endeavor">Endeavor</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
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
