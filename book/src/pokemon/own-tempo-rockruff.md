<details class="pokemon-card-container">
<summary>Own Tempo Rockruff (#105)</summary>
Types: Rock • Egg Groups: Field

<div class="pokemon-card">
<div class="card-column">
**Abilities**
- Own Tempo

**Type Matchups**

*Resists / Immune to*
- Normal (0.5×)
- Fire (0.5×)
- Poison (0.5×)
- Flying (0.5×)

*Weak to*
- Water (2×)
- Grass (2×)
- Fighting (2×)
- Ground (2×)
- Steel (2×)

**TM/HM Moves**
- <a href="move-lookup.html?q=tm06-toxic">TM06 - Toxic</a>
- <a href="move-lookup.html?q=tm12-taunt">TM12 - Taunt</a>
- <a href="move-lookup.html?q=tm17-protect">TM17 - Protect</a>
- <a href="move-lookup.html?q=tm28-dig">TM28 - Dig</a>
- <a href="move-lookup.html?q=tm32-double-team">TM32 - Double Team</a>
- <a href="move-lookup.html?q=tm37-sandstorm">TM37 - Sandstorm</a>
- <a href="move-lookup.html?q=tm39-rock-tomb">TM39 - Rock Tomb</a>
- <a href="move-lookup.html?q=tm42-facade">TM42 - Facade</a>
- <a href="move-lookup.html?q=tm44-rest">TM44 - Rest</a>
- <a href="move-lookup.html?q=tm45-attract">TM45 - Attract</a>
- <a href="move-lookup.html?q=tm49-bulldoze">TM49 - Bulldoze</a>
- <a href="move-lookup.html?q=tm55-snarl">TM55 - Snarl</a>
</div>
<div class="card-column">
<label><input type="checkbox" class="caught-check" data-species="own-tempo-rockruff" /> Caught</label>

**Base Stats**

| Stat | Value |
| --- | --- |
| HP | <span class="stat-value stat-low">45</span> |
| Attack | <span class="stat-value stat-mid">65</span> |
| Defense | <span class="stat-value stat-low">40</span> |
| Sp. Atk | <span class="stat-value stat-low">30</span> |
| Sp. Def | <span class="stat-value stat-low">40</span> |
| Speed | <span class="stat-value stat-mid">60</span> |
| Total | <span class="stat-value stat-low">280</span> |

**Level-Up Moves**
- <a href="move-lookup.html?q=tackle">Tackle</a> (Lv 1)
- <a href="move-lookup.html?q=leer">Leer</a> (Lv 1)
- <a href="move-lookup.html?q=sand-attack">Sand Attack</a> (Lv 4)
- <a href="move-lookup.html?q=bite">Bite</a> (Lv 7)
- <a href="move-lookup.html?q=howl">Howl</a> (Lv 12)
- <a href="move-lookup.html?q=rock-throw">Rock Throw</a> (Lv 15)
- <a href="move-lookup.html?q=odor-sleuth">Odor Sleuth</a> (Lv 18)
- <a href="move-lookup.html?q=rock-tomb">Rock Tomb</a> (Lv 23)
- <a href="move-lookup.html?q=roar">Roar</a> (Lv 26)
- <a href="move-lookup.html?q=stealth-rock">Stealth Rock</a> (Lv 29)
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a> (Lv 34)
- <a href="move-lookup.html?q=scary-face">Scary Face</a> (Lv 37)
- <a href="move-lookup.html?q=crunch">Crunch</a> (Lv 40)
- <a href="move-lookup.html?q=rock-climb">Rock Climb</a> (Lv 45)
- <a href="move-lookup.html?q=stone-edge">Stone Edge</a> (Lv 48)

**Egg Moves**
- <a href="move-lookup.html?q=crush-claw">Crush Claw</a>
- <a href="move-lookup.html?q=fire-fang">Fire Fang</a>
- <a href="move-lookup.html?q=thunder-fang">Thunder Fang</a>
- <a href="move-lookup.html?q=sucker-punch">Sucker Punch</a>
- <a href="move-lookup.html?q=thrash">Thrash</a>

**Tutor Moves**
- <a href="move-lookup.html?q=body-slam">Body Slam</a>
- <a href="move-lookup.html?q=double-edge">Double-Edge</a>
- <a href="move-lookup.html?q=endure">Endure</a>
- <a href="move-lookup.html?q=mud-slap">Mud-Slap</a>
- <a href="move-lookup.html?q=rock-slide">Rock Slide</a>
- <a href="move-lookup.html?q=sleep-talk">Sleep Talk</a>
- <a href="move-lookup.html?q=snore">Snore</a>
- <a href="move-lookup.html?q=swagger">Swagger</a>
- <a href="move-lookup.html?q=swords-dance">Swords Dance</a>
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
